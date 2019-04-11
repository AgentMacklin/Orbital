/**
 * Handles julian and gregorian date stuff
 */

pub struct Gregorian {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
}

impl std::fmt::Display for Gregorian {
    // Tell Rust how to print out Gregorian structs
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\
             Year:      {}\n\
             Month:     {}\n\
             Day:       {}\n\
             Hour:      {}\n\
             Minutes:   {}\n\
             Seconds:   {:.2}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
/**
 * Convert julian date to gregorian, returns a Gregorian struct
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
        leap_years = 0.25 * (year - 1900.0 - 1.0);
        days = (julian - 2_415_019.5) - (365.0 * (year - 1900.0) + leap_years);
    }
    if year % 4.0 == 0.0 {
        l_months[2] = 29.0;
    }
    let day_of_year = days.trunc();
    let mut sum_days = 0.0;
    let mut i = 0;
    while sum_days + 1.0 < day_of_year {
        sum_days += l_months[i];
        i += 1;
    }
    let month = i as f64;
    let (l_month_sum, _) = l_months.split_at(i - 1 as usize);
    let m_sum: f64 = l_month_sum.iter().sum();
    let day: f64 = day_of_year - m_sum;
    let tau: f64 = (days - day_of_year) * 24.0;
    let hour = tau.trunc();
    let minute = ((tau - hour) * 60.0).trunc();
    let second = (tau - hour - (minute / 60.0)) * 3600.0;
    Gregorian {
        year: year as u32,
        month: month as u32,
        day: day as u32,
        hour: hour as u32,
        minute: minute as u32,
        second: second,
    }
}
