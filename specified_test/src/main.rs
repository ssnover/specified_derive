use specified::Specified;

#[allow(dead_code)]
#[derive(Specified)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("{}", Point::specified());
}
