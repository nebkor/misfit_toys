use std::{error::Error, fmt::Display};

use chrono::{DateTime, Duration, Utc};
use dateparser::DateTimeUtc;

const DEFAULT: &str = "1776-07-04T12:00:00-04:00";

pub type Freesult = Result<FreedomDate, FreedomError>;

#[derive(Debug, Clone)]
pub enum FreedomError {
    MoonLanguage(String),
    PreCreation(String),
}

impl Display for FreedomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreedomError::MoonLanguage(s) => {
                write!(f, "I don't speak your crazy moon-language! '{s}'")
            }
            FreedomError::PreCreation(s) => write!(
                f,
                "That doesn't hardly make any sense, '{s}' is before the start of time.",
            ),
        }
    }
}

impl Error for FreedomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub trait FreedomTime {
    fn epoch(&self) -> u64;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreedomDate {
    bang: DateTime<Utc>,
    big_bang: DateTime<Utc>,
}

impl Default for FreedomDate {
    fn default() -> Self {
        Self {
            bang: DateTime::parse_from_rfc3339(DEFAULT).unwrap().into(),
            big_bang: DateTime::parse_from_rfc3339(DEFAULT).unwrap().into(),
        }
    }
}

impl Display for FreedomDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bang.format("%-m/%-d/%-y"))
    }
}

impl std::ops::Add<Duration> for FreedomDate {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            big_bang: self.big_bang,
            bang: self.bang + rhs,
        }
    }
}

impl std::ops::Sub for FreedomDate {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.bang - rhs.bang
    }
}

impl FreedomDate {
    pub fn liberate(datestring: &str) -> Freesult {
        let bang = if let Ok(bang) = datestring.parse::<DateTimeUtc>() {
            bang
        } else {
            return Err(FreedomError::MoonLanguage(datestring.to_owned()));
        };
        let big_bang = DEFAULT.parse::<DateTimeUtc>().unwrap();

        FreedomDate::new(bang.0, big_bang.0, datestring)
    }

    fn new(bang: DateTime<Utc>, big_bang: DateTime<Utc>, datestring: &str) -> Freesult {
        if bang < big_bang {
            return Err(FreedomError::PreCreation(datestring.to_owned()));
        }

        Ok(FreedomDate { bang, big_bang })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_birthday() {
        let result = format!("{}", FreedomDate::default());
        assert_eq!(result, "7/4/76");
    }

    #[test]
    fn no_time_before_time() {
        let result = FreedomDate::liberate("1774-01-01");
        assert!(match result {
            Err(FreedomError::PreCreation(_)) => true,
            _ => false,
        });
    }
}
