fn julian_to_greg(julian: f64) -> (f64, f64, f64, f64, f64, f64) {
        let l_months = vec![31.0, 28.0, 31.0, 30.0, 31.0, 30.0, 31.0, 31.0, 30.0, 31.0, 30.0, 31.0];
        let t_1900 = (julian - 2_415_019.5) / 365.25;
        let mut year = 1900 + t_1900.trunc(); // <- convenient that Rust has a trunc method for floats
        let mut leap_years = (0.25 * (year - 1900 - 1)).trunc();
        let mut days = (julian - 2_415_019.5) - (365.0 * (year - 1900) + leap_years);
        if days < 1.0 {
            years -= 1;
            leap_years = (0.25 * (year - 1900 - 1));
            days = ($julian - 2_415_019.5) - (365.0 * (year - 1900) + leap_years);
        } if year % 4 == 0 {
            l_months[2] = 29.0;
        }
        let day_of_year = days.trunc();
        let sum_days = 0.0;
        let i = 0;
        let month = while sum_days + 1.0 < day_of_year {
            sum_days += l_months[i];
            i += 1;
        };
        let day = day_of_year - l_months.iter().sum();
        let tau = (day - day_of_year) * 24.0;
        let hour = tau.trunc();
        let minute = ((tau - hour) * 60).trunc();
        let second = ((tau - hour - (minute / 60)) * 3600).trunc();
        (year, month, day, hour, minute, second)
}
/* Macro that makes it more convenient to print out results */
macro_rules! printer {
    // print vector
    ($msg:expr, v => $val:expr) => {
        let units = ["x", "y", "z"];
        println!("{}:", $msg.cyan());
        for (index, element) in $val.iter().enumerate() {
            println!("  {}: {:.10e}", units[index].green(), element);
        }
        println!();
    };
    // print scalar in scientific notation (s => denotes $val is a scalar and not a vector)
    ($msg:expr, s => $val:expr) => {
        println!("{}:\n  {:.10e}\n", $msg.green(), $val)
    };

    // print matrix
    ($msg:expr, m => $val:expr) => {
        println!("{}:", $msg.cyan());
        for i in 0..3 {
            let row = $val.row(i);
            println!("{:>13.6e} {:>13.6e} {:>13.6e}", row[0], row[1], row[2]);
        }
        println!();
    };
}

macro_rules! date {
    // Going from Gregorian date to Julian date
    ($year:expr, $month:expr, $day:expr, $hours:expr, $minutes:expr, $seconds:expr) => {
        367.0 * $year as f64 - ((7.0 * ($month as f64 + 9.0) / 12.0) / 4.0).floor()
        + ((275.0 * $month as f64) / 9.0).floor()
        + 1_721_013.5
        + $day as f64
        + ((((($seconds as f64 / 60.0) + $minutes as f64) / 60.0) + $hours as f64) / 24.0).floor();
    };
    // Going from Julian to Gregorian
    ($julian:expr) => {
        julian_to_greg($julian);
    };
}
