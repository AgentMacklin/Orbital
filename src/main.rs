mod util;
use self::util::orbitable::Body;

fn main() {
    let mercury = Body::new(3.302e23, 2440e3, None);
    let venus = Body::new(4.8685e24, 6051.8e3, None);
    let earth = Body::new(5.97219e24, 6371.01e3, None);
    let mars = Body::new(6.4185e23, 3389.9e3, None);
    let vesta = Body::new(2.59076e20, 262.7e3, None);
    let ceres = Body::new(9.38416e20, 470e3, None);
    let jupiter = Body::new(1.89813e27, 69911e3, None);
    let saturn = Body::new(5.68319e26, 58232e3, None);
    let uranus = Body::new(8.68103e25, 25362e3, None);
    let neptune = Body::new(1.02e26, 24624e3, None);
    let pluto = Body::new(1.307e22, 1195e3, None);

    let planets = [
        mercury, venus, earth, mars, vesta, ceres, jupiter, saturn, uranus, neptune, pluto,
    ];

    let names = [
        "Mercury", "Venus", "Earth", "Mars", "Vesta", "Ceres", "Jupiter", "Saturn", "Uranus",
        "Neptune", "Pluto",
    ];

    println!("Surface Gravity:");
    for (i, p) in planets.into_iter().enumerate() {
        println!("{}: {:.12} m/s^2", names[i], p.surface_gravity());
    }
}
