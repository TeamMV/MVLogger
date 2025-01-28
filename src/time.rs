pub(crate) fn convert_epoch_to_datetime(epoch_millis: u128) -> String {
    let seconds_since_epoch = epoch_millis / 1000;
    let seconds = seconds_since_epoch % 60;
    let minutes = (seconds_since_epoch / 60) % 60;
    let hours = (seconds_since_epoch / 3_600) % 24;
    let days_since_epoch = seconds_since_epoch / 86_400;

    let mut year = 1970;
    let mut days_remaining = days_since_epoch;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days_remaining < days_in_year {
            break;
        }
        days_remaining -= days_in_year;
        year += 1;
    }

    let (month, day) = get_month_and_day(days_remaining as u32, year);

    format!(
        "{:02}/{:02}/{:02} {:02}:{:02}:{:02}",
        day,
        month,
        year % 100,
        hours,
        minutes,
        seconds
    )
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn get_month_and_day(mut days: u32, year: i32) -> (u32, u32) {
    let mut month = 1;
    let days_in_month = [
        31,
        if is_leap_year(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    for &dim in days_in_month.iter() {
        if days < dim {
            break;
        }
        days -= dim;
        month += 1;
    }
    (month, days + 1)
}
