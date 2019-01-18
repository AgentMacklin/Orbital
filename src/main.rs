mod util;
use self::util::orbitable::{Body, Orbit};

fn main() {
    let Earth = Body::new(5.974e24, 6378e3, None);
    println!("{:.12} m/s{}", Earth.surface_gravity(), 0x00B2 as char);
}
