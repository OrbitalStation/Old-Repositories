use crate::{Σ, DEFAULT_PRECISION};
use core::ops::*;

///
/// Find a <a href="https://en.wikipedia.org/wiki/Logarithm">Logarithm</a>
/// 
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn log <T> (a: T, b: T, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const AddAssign +
    ~const Sub <Output = T> +
    ~const Add <Output = T> +
    ~const Div <Output = T> +
    Copy
{
    ln(a, precision) / ln(b, precision)
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Binary_logarithm">Binary logarithm</a>
/// 
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
/// 
pub const fn lb <T> (x: T, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const AddAssign +
    ~const Sub <Output = T> +
    ~const Add <Output = T> +
    ~const Div <Output = T> +
    Copy
{
    ln(x, precision) * T::from(core::f32::consts::LOG2_E)
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Common_logarithm">Common(decimal) logarithm</a>
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
/// 
pub const fn lg <T> (x: T, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const AddAssign +
    ~const Sub <Output = T> +
    ~const Add <Output = T> +
    ~const Div <Output = T> +
    Copy
{
    ln(x, precision) * T::from(core::f32::consts::LOG10_E)
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Natural_logarithm">Natural logarithm</a>
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
/// 
pub const fn ln <T> (x: T, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const AddAssign +
    ~const Sub <Output = T> +
    ~const Add <Output = T> +
    ~const Div <Output = T> +
    Copy
{
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x
    };

    let x = (x - T::from(1.0)) / (x + T::from(1.0));
    let xx = x * x;
    let mut fraction = x;
    let mut cur_div = T::from(1.0);

    T::from(2.0) * Σ!(precision => {
        let calculated = fraction / cur_div;
        cur_div += T::from(2.0);
        fraction *= xx;
        calculated
    })
}
