#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate specified_derive;
trait Specified {
    fn specified() -> String;
}
struct Point {
    x: f64,
    y: f64,
}
impl Specified for Point {
    fn specified() -> std::string::String {
        let mut spec_string = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", " {\n"],
                &match (&Point,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        };
        let member_string = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ": "],
                &match (&"x", &"f64") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        spec_string.push_str(&member_string);
        let member_string = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ": "],
                &match (&"y", &"f64") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        spec_string.push_str(&member_string);
        let end_brace = "}";
        spec_string.push_str(&end_brace);
        spec_string
    }
}
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&Point::specified(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
}
