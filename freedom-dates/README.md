# Freedom Dates

## *Let no man put afunder what I believe is thee proper way to shew a written Date.* --General George Washington

The `freedom-dates` crate provides a convenient suite of affordances for working with dates in
*freedom format*. That is, it takes representations of dates in Communinst formats like
"2023-02-08", and liberates them into a Freedom format like "2/8/23".

``` rust
    let communism = "2023-02-08T12:00:00-07:00";

    let freedom = FreedomDate::liberate(communism).unwrap();
    
    assert_eq!("2/7/23", &result.to_string());
```

Since Freedom was born on the Fourth of July, 1776 (*7/4/76*), dates before then are neither valid
nor representable. If you attempt to liberate a date like that, you'll get a
`FreedomError::PreCreation` error. Attempts to liberate a date that is *too* Communist will result
in a `FreedomError::TooCommunist` error.

``` rust
    let bad_communism = "Comrade, today is the eighth of Februrary, in the year 2023.";
    let pre_history = "1775-07-04";

    let too_communist = FreedomDate::liberate(bad_communism).unwrap_err();
    let pre_historic_nonsense = FreedomDate::liberate(pre_history).unwrap_err();

    println!("'{bad_communism}' is impossible to comprehend: `{too_communist}`\n");

    println!("'{pre_history}' is not a real date: `{pre_historic_nonsense}`\n");

```

will print out:


> 'Comrade, today is the eighth of Februrary, in the year 2023.' is impossible to comprehend: `I
> don't speak your crazy Communism-language! 'Comrade, today is the eighth of Februrary, in the year
> 2023.'`

> '1775-07-04' is not a real date: `That doesn't hardly make no sense, '1775-07-04' is before the
> very start of Time/Freedom itself.`

A `freedomstamp` is the number of seconds since the birth of Freedom, and positive integer values
can be turned into FreedomDates:

``` rust
    // `From<u64>` is implemented for FreedomDates
    let birthday_of_freedom: FreedomDate = 0.into();
    println!("The Birthday of Freedom is {birthday_of_freedom}\n");
```

will print

> The Birthday of Freedom is 7/4/76

and

``` rust
    let one_day = Duration::days(1);
    let day_after_freedom = birthday_of_freedom + one_day;
    println!(
        "The day after Freedom was born, {day_after_freedom}, {} seconds had passed.",
        day_after_freedom.freedomstamp()
    );
```

will print

> The day after Freedom was born, 7/5/76, 86400 seconds had passed.

These examples can be found in [the tour](./examples/tour.rs).

## Versioning

Freedom *STARTS* at number 1, baby! And every release is ten times the last, so second release is
10, then 100, etc. FREEDOM!

## License

This software is released under the [Chaos License](./LICENSE.md)
