#[macro_use]
extern crate specified_derive;

trait Specified {
    fn specified() -> String;
}

#[derive(Specified)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("{}", Point::specified());
}
