/**
 * Macros that make some operations more convenient.
 */

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
        date::julian_to_greg($julian);
    };
}
