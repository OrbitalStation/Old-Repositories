use crate::{Σ, DEFAULT_PRECISION};
use core::ops::*;
use crate::ln;

///
/// Find <a href="https://en.wikipedia.org/wiki/Exponentiation">`n`-th power of `x`</a>
///
pub const fn pow <T: Copy + ~const MulAssign> (x: T, n: usize) -> T {
    let mut result = x;
    let mut i = 1;
    while i < n {
        result *= x;
        i += 1
    }
    result
}

pub const fn powf <T> (x: T, a: f32, precision: Option <usize>) -> T
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
    exp(T::from(a) * ln(x, precision), precision)
}

pub const fn exp <T> (x: T, precision: Option <usize>) -> T
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

    let mut n = T::from(0.0);

    let mut a = T::from(1.0);
    let mut b = T::from(1.0);

    Σ!(precision => {
        let calculated = a / b;
        a *= x;
        n += T::from(1.0);
        b *= n;
        calculated
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn powf_val() {
        assert_eq!(powf(0.0, 0.0, None), 1.0);
        assert_eq!(powf(2.6, 4.1, None), 50.279469537021214);
    }

    #[test]
    fn exp_val() {
        assert_eq!(exp(0.0, None), 1.0);
        assert_eq!(exp(1.0, None), 2.7182818284590455);
        assert_eq!(exp(-1.0, None), 0.36787944117144245);
        assert_eq!(exp(6.0, None), 403.4287934927351);
    }
}
