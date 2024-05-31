# Const-trig

This crate exists to provide `const` versions of functions,
such as `cos`, `sin`, etc.

Note that this crate requires nightly Rust
to unify behaviour of functions.

Crate is no longer needed when all this function will
be const in std.

Any function, such as `cos`, `sin` from std
can be accessed as `const_trig::cos`.

There aren't all trig functions,
but if you want to add them, you can open a <a href="https://github.com/Roman-Tarasenko-27/const-trig/issues">GitHub issue</a>
(someday I will read it) :)
Usage:

 ```rust
use const_trig::{ToDegrees, ToRadians};

macro_rules! cmp {
    ($fn:ident, $e:expr) => {
        println!(concat!("const_trig::", stringify!($fn), " = {}"), const_trig::$fn($e, None));
        println!(concat!("std ", stringify!($fn), " = {}\n"), $e.$fn());
    };

    ($fn:ident, $lib:expr, $std:expr) => {
        println!(concat!("const_trig::", stringify!($fn), " = {}"), const_trig::$fn($lib, None));
        println!(concat!("std ", stringify!($fn), " = {}\n"), $std.$fn());
    };

    (! $fn:ident, $e:expr, $fnstd:ident) => {
        println!(concat!("const_trig::", stringify!($fn), " = {}"), const_trig::$fn($e, None));
        println!(concat!("std ", stringify!($fnstd), " = {}\n"), $e.$fnstd());
    };
}

fn main() {
    let sixty_degrees = 60.0f32.degrees().radians().get();
    cmp!(sin, sixty_degrees.radians(), sixty_degrees);
    cmp!(cos, sixty_degrees.radians(), sixty_degrees);

    cmp!(sqrt, 4.0f64);
    cmp!(sqrt, 2.0f64);

    cmp!(ln, 10.0f64);
    cmp!(ln, core::f64::consts::E);

    cmp!(! lg, core::f64::consts::E, log10);

    cmp!(! lb, 10.0f64, log2);

    println!("const_trig::log = {}", const_trig::log(3.5, 10.0f64, None));
    println!("std log = {}", 3.5f64.log(10.0));

    println!("{}", const_trig::root(34.0, 5, None))
}
 ```
