extern crate colored;
use colored::*;

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

    // Print an integer
    ($msg:expr, i => $val:expr) => {
        println!("{}:\n  {}\n", $msg.green(), $val)
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
        (367.0 * $year as f64) - ((7.0 * (($month as f64 + 9.0) / 12.0).floor()) / 4.0).floor()
            + ((275.0 * $month as f64) / 9.0).floor()
            + 1_721_013.5
            + $day as f64
            + ((((($seconds as f64 / 60.0) + $minutes as f64) / 60.0) + $hours as f64) / 24.0);
    };
    // Going from Julian to Gregorian, which needs it's own function since macros don't like
    // lets when you're trying to return data from the macro
    ($julian:expr) => {
        self::macros::julian_to_greg($julian);
    };
}

/**
 * yikes, I'm not sure how I ever got this to work
 */
pub fn julian_to_greg(julian: f64) -> Gregorian {
    let mut l_months = vec![
        31.0, 28.0, 31.0, 30.0, 31.0, 30.0, 31.0, 31.0, 30.0, 31.0, 30.0, 31.0,
    ];
    let t_1900 = (julian - 2_415_019.5) / 365.25;
    let mut year = 1900.0 + t_1900.trunc(); // <- how convenient
    let mut leap_years = (0.25 * (year - 1900.0 - 1.0)).trunc();
    let mut days = (julian - 2_415_019.5) - (365.0 * (year - 1900.0) + leap_years);
    if days < 1.0 {
        year -= 1.0;
        leap_years = (0.25 * (year - 1900.0 - 1.0));
        days = (julian - 2_415_019.5) - (365.0 * (year - 1900.0) + leap_years);
    }
    if year % 4.0 == 0.0 {
        l_months[2] = 29.0;
    }
    let day_of_year = days.trunc();
    let mut sum_days = 0.0;
    let mut i = 0;
    let mut month = 0.0;
    while sum_days + 1.0 < day_of_year {
        sum_days += l_months[i];
        i += 1;
    }
    month = i as f64;
    let (l_month_sum, _) = l_months.split_at(i - 1 as usize);
    let m_sum: f64 = l_month_sum.iter().sum();
    let day: f64 = day_of_year - m_sum;
    let tau: f64 = (days - day_of_year) * 24.0;
    let hour = tau.trunc();
    let minute = ((tau - hour) * 60.0).trunc();
    let second = ((tau - hour - (minute / 60.0)) * 3600.0).trunc();
    Gregorian {
        year: year as u32,
        month: month as u32,
        day: day as u32,
        hour: hour as u32,
        minute: minute as u32,
        second: second as u32,
    }
}

pub struct Gregorian {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl std::fmt::Display for Gregorian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\n  \
             Year:    {:>4}\n  \
             Month:   {:>4}\n  \
             Day:     {:>4}\n  \
             Hour:    {:>4}\n  \
             Minutes: {:>4}\n  \
             Seconds: {:>4}",
            "Gregorian Date:".cyan(),
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}
