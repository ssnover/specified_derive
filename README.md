# specified
A derive macro trait for generating a `String` representation of a struct.

Usage:
```rust
#[derive(Specified)]
struct Point {
    x: f64,
    y: f64,
}

println!("{}", Point::specified());
```

Outputs:
```
Point {
    x: f64
    y: f64
}
```