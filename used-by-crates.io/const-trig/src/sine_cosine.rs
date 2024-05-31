use core::ops::*;
use crate::{DEFAULT_PRECISION, Σ, Radians};

///
/// Find <a href="https://en.wikipedia.org/wiki/Sine">Sine</a>.
///
/// Note that `rads` is radians, not degrees.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn sin <T> (rads: Radians <T>, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const DivAssign +
    ~const Add <Output = T> +
    ~const AddAssign +
    ~const Neg <Output = T> +
    Copy
{
    sine_cosine_helper(rads, precision, rads.get(), T::from(1.0))
}

///
/// Find <a href="https://en.wikipedia.org/wiki/Cosine">Cosine</a>.
///
/// Note that `rads` is radians, not degrees.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn cos <T> (rads: Radians <T>, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const DivAssign +
    ~const Add <Output = T> +
    ~const AddAssign +
    ~const Neg <Output = T> +
    Copy
{
    sine_cosine_helper(rads, precision, T::from(1.0), T::from(0.0))
}

const fn sine_cosine_helper <T> (rads: Radians <T>, precision: Option <usize>, mut fraction: T, mut current_fac: T) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const DivAssign +
    ~const Add <Output = T> +
    ~const AddAssign +
    ~const Neg <Output = T> +
    Copy
{
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x
    };

    let rads = rads.get();

    let neg_x_squared = -(rads * rads);

    Σ!(precision => {
        let calculated = fraction;
        fraction *= neg_x_squared;
        fraction /= (current_fac + T::from(1.0)) * (current_fac + T::from(2.0));
        current_fac += T::from(2.0);
        calculated
    })
}
