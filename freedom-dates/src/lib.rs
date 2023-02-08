use std::{error::Error, fmt::Display};

pub use chrono::{DateTime as CommieDateTime, Duration, Utc as FancyTime};
use dateparser::DateTimeUtc;

const FREEDOM_FMT: &str = "%-m/%-d/%-y";
//const FOUR_SCORE: u64 = 10_098_216_320; // seconds in 80 years

/// Freedom was born at noon on the Fourth of July, '76, Eastern Time. This is History.
pub const FREEDOMS_BIRTHDAY: &str = "1776-07-04T12:00:00-04:00";

/// A Result of Freedom.
pub type Freesult = Result<FreedomDate, FreedomError>;

/// Either your date string makes no sense because it's too Communist, or because it refers to some
/// impossible date that is ... "before" the start of Time/Freedom itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreedomError {
    TooCommunist(String),
    PreCreation(String),
}

impl Display for FreedomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreedomError::TooCommunist(s) => {
                write!(f, "I don't speak your crazy Communism-language! '{s}'")
            }
            FreedomError::PreCreation(s) => write!(
                f,
                "That doesn't hardly make no sense, '{s}' is before the very start of Time/Freedom itself.",
            ),
        }
    }
}

impl Error for FreedomError {}

/// FreedomTime is aware of the Birthday of Freedom (July 4, '76).
pub trait FreedomTime {
    /// Number of whole seconds since the Birthday of Freedom.
    fn freedomstamp(&self) -> u64;
}

/// A FreedomDate is the One True Date representation. All other models are Communist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreedomDate {
    shot_heard_around_the_world: CommieDateTime<FancyTime>,
    shot_heard_around_the_universe: CommieDateTime<FancyTime>,
}

/// By default, FreedomDates are July 4th, '76.
impl Default for FreedomDate {
    fn default() -> Self {
        Self {
            shot_heard_around_the_world: CommieDateTime::parse_from_rfc3339(FREEDOMS_BIRTHDAY)
                .unwrap()
                .into(),
            shot_heard_around_the_universe: CommieDateTime::parse_from_rfc3339(FREEDOMS_BIRTHDAY)
                .unwrap()
                .into(),
        }
    }
}

impl Display for FreedomDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.shot_heard_around_the_world.format(FREEDOM_FMT)
        )
    }
}

impl std::ops::Add<Duration> for FreedomDate {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            shot_heard_around_the_world: self.shot_heard_around_the_world + rhs,
            shot_heard_around_the_universe: self.shot_heard_around_the_universe,
        }
    }
}

impl std::ops::Sub<Duration> for FreedomDate {
    type Output = Freesult;

    fn sub(self, rhs: Duration) -> Self::Output {
        let datestring = self.shot_heard_around_the_world - rhs;
        let datestring = format!("{}", datestring.format(FREEDOM_FMT));
        FreedomDate::new(
            self.shot_heard_around_the_world - rhs,
            self.shot_heard_around_the_universe,
            &datestring,
        )
    }
}

impl std::ops::Sub<Self> for FreedomDate {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.shot_heard_around_the_world - rhs.shot_heard_around_the_world
    }
}

impl FreedomTime for FreedomDate {
    fn freedomstamp(&self) -> u64 {
        (self.shot_heard_around_the_world - self.shot_heard_around_the_universe).num_seconds()
            as u64
    }
}

/// A FreedomDate that is `value` seconds after the Birthday of Freedom.
impl From<u64> for FreedomDate {
    fn from(value: u64) -> Self {
        let f = FreedomDate::default();
        let dur = Duration::seconds(value as i64);
        f + dur
    }
}

impl FreedomDate {
    /// To liberate a representation of a date is to make it Free. But the tree of FreeDates must
    /// occasionally be watered with the blood of badly-formed datestrings, and here is where the
    /// true test of Datriots is found.
    pub fn liberate(datestring: &str) -> Freesult {
        let bang = if let Ok(bang) = datestring.parse::<DateTimeUtc>() {
            bang
        } else {
            return Err(FreedomError::TooCommunist(datestring.to_owned()));
        };
        let big_bang = FREEDOMS_BIRTHDAY.parse::<DateTimeUtc>().unwrap();

        FreedomDate::new(bang.0, big_bang.0, datestring)
    }

    fn new(
        bang: CommieDateTime<FancyTime>,
        big_bang: CommieDateTime<FancyTime>,
        datestring: &str,
    ) -> Freesult {
        if bang < big_bang {
            Err(FreedomError::PreCreation(datestring.to_owned()))
        } else {
            Ok(FreedomDate {
                shot_heard_around_the_world: bang,
                shot_heard_around_the_universe: big_bang,
            })
        }
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

    #[test]
    fn i_dont_speak_your_crazy_moon_language() {
        let result = FreedomDate::liberate("this is not a datestring of honor");
        assert!(match result {
            Err(FreedomError::TooCommunist(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn a_regular_date_that_is_fine() {
        let result = FreedomDate::liberate("2023-02-07T12:00:00-07:00").unwrap();
        assert_eq!("2/7/23", &result.to_string());
    }

    #[test]
    fn cant_trick_me_with_subtraction_wizard() {
        let result = FreedomDate::default() - chrono::Duration::days(2);
        assert!(match result {
            Err(FreedomError::PreCreation(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn one_day_after_americas_birthday() {
        let hangover = FreedomDate::default() + chrono::Duration::days(1);
        let seconds = 24 * 60 * 60;
        assert_eq!(hangover.freedomstamp(), seconds);
    }
}
