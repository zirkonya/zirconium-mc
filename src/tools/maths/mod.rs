pub mod ops;
pub mod varint;
pub mod vector;

/// The function `pseudo_random` generates a pseudo-random number using the given parameters.
/// 
/// Arguments:
/// 
/// * `a`: The parameter `a` is an unsigned 64-bit integer.
/// * `b`: The parameter `b` in the `pseudo_random` function is an input of type `u64`. It represents
/// one of the values used in the pseudo-random number generation algorithm.
/// * `seed`: The `seed` parameter in the `pseudo_random` function is used as an initial value to
/// generate pseudo-random numbers. It is an unsigned 64-bit integer (`u64`) that determines the
/// starting point for the random number generation algorithm.
/// 
/// Returns:
/// 
/// The function `pseudo_random` returns a `u64` value.
pub fn pseudo_random(a: u64, b: u64, seed: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    const W: u64 = 8 * std::mem::size_of::<u64>() as u64;
    const S: u64 = W / 2;
    a = a.wrapping_mul(3284157443).wrapping_add(seed);
    b ^= a << S | a >> W - S;
    b = b.wrapping_mul(1911520717);
    a ^= b << S | b >> W - S;
    a = a.wrapping_mul(2048419325);
    a
}

/// The sigmoid function calculates the sigmoid value of a given input.
/// 
/// Arguments:
/// 
/// * `x`: The parameter `x` is a floating-point number (f64) that represents the input value for the
/// sigmoid function.
/// 
/// Returns:
/// 
/// The function `sigmoid` returns a `f64` (floating-point number) which is the result of applying the
/// sigmoid function to the input `x`.
pub fn sigmoid(x: f64) -> f64 {
    1f64 / (1f64 + f64::exp(-x))
}

/// The `smin` function calculates the smooth minimum of two values with a given factor.
/// 
/// Arguments:
/// 
/// * `a`: The parameter `a` is the first value to compare.
/// * `b`: The parameter `b` represents the upper bound value.
/// * `factor`: The `factor` parameter in the `smin` function represents the smoothness factor. It
/// determines how smooth the interpolation between `a` and `b` should be. A higher value of `factor`
/// will result in a smoother interpolation, while a lower value will result in a sharper transition.
/// 
/// Returns:
/// 
/// The function `smin` returns a `f64` (a 64-bit floating-point number).
pub fn smin(a: f64, b: f64, factor: f64) -> f64 {
    let h = ((b - a + factor) / (2.0 * factor)).clamp(0.0, 1.0);
    a * h + b * (1.0 - h) - factor * h * (1.0 - h)
}

/// The `gradient` function returns the cosine and sine of a given angle.
/// 
/// Arguments:
/// 
/// * `a`: f64 - a floating-point number representing an angle in radians.
pub fn gradient(a: f64) -> (f64, f64) {
    (a.cos(), a.sin())
}

