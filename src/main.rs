#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("Hello, World!");
}
