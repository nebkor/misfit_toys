use freedom_dates::*;

fn main() {
    let good_communism = "2023-02-08T12:00:00-07:00";
    let bad_communism = "Comrade, today is the eighth of Februrary, in the year 2023.";
    let pre_history = "1775-07-04";

    let liberated = FreedomDate::liberate(good_communism).unwrap();
    let too_communist = FreedomDate::liberate(bad_communism).unwrap_err();
    let pre_historic_nonsense = FreedomDate::liberate(pre_history).unwrap_err();

    println!("'{good_communism}' is liberated: `{liberated}`\n");
    println!("'{bad_communism}' is impossible to comprehend: `{too_communist}`\n");
    println!("'{pre_history}' is not a real date: `{pre_historic_nonsense}`\n");

    // `From<u64>` is implemented for FreedomDates
    let birthday_of_freedom: FreedomDate = 0.into();
    println!("The Birthday of Freedom is {birthday_of_freedom}\n");

    // FreedomDates implement the FreedomTime trait
    let one_day = Duration::days(1);
    let day_after_freedom = birthday_of_freedom + one_day;
    println!(
        "The day after Freedom was born, {day_after_freedom}, {} seconds had passed.",
        day_after_freedom.freedomstamp()
    );
}
