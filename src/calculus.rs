/// Calculates the derivative of a function at a given point using numerical differentiation.
///
/// This function approximates the derivative of a given function at a specific point `x` using the
/// central difference method. Numerical differentiation is useful for approximating derivatives
/// when an analytical expression for the derivative is not available or when working with discrete
/// data points. The accuracy of the approximation depends on the step size `h`, where smaller values
/// of `h` generally provide more accurate results but may introduce numerical instability if too small.
///
/// # Parameters
///
/// - `func`: A closure or function that represents the mathematical function whose derivative is to
///   be calculated. The function should accept a single `f64` argument and return an `f64` value.
/// - `x`: A floating-point value representing the point at which the derivative is to be calculated.
/// - `h`: A floating-point value representing the step size used for the numerical differentiation.
///   The value of `h` should be small but not too small to avoid numerical instability.
///
/// # Returns
///
/// Returns the approximate value of the derivative of the function at the point `x`.
///
/// # Examples
///
/// ```
/// fn main() {
///     // Define a function for which we want to compute the derivative.
///     let func = |x: f64| x.powi(2); // f(x) = x^2
///
///     // Calculate the derivative of the function at x = 1.0 with a step size of 0.01.
///     let result = derivative(func, 1.0, 0.01);
///     println!("The derivative at x = 1.0 is approximately: {}", result);
/// }
/// ```
///
/// # Notes
///
/// - The choice of step size `h` can significantly affect the accuracy of the derivative calculation.
/// - If `h` is too large, the approximation may be inaccurate. If `h` is too small, numerical errors
///   may dominate. Experimentation with different values of `h` is recommended to achieve the best
///   balance between accuracy and numerical stability.
pub fn derivative<F>(func: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (func(x + h) - func(x - h)) / (2.0 * h)
}

/// Calculates the integral of a function using the trapezoidal rule.
///
/// This function approximates the definite integral of a given function over the interval `[a, b]`
/// using the trapezoidal rule. The trapezoidal rule is a numerical integration technique that estimates
/// the area under the curve by dividing the interval into `n` subintervals and approximating the area of
/// each subinterval as a trapezoid. This method is useful for evaluating integrals when an analytical
/// solution is difficult to obtain or when working with discrete data points.
///
/// # Parameters
///
/// - `func`: A closure or function that represents the mathematical function to be integrated. The
///   function should accept a single `f64` argument and return an `f64` value.
/// - `a`: A floating-point value representing the lower bound of the integration interval.
/// - `b`: A floating-point value representing the upper bound of the integration interval.
/// - `n`: An unsigned integer representing the number of subintervals into which the interval `[a, b]`
///   is divided. A larger value of `n` generally provides a more accurate approximation.
///
/// # Returns
///
/// Returns the approximate value of the integral of the function over the interval `[a, b]`.
///
/// # Examples
///
/// ```
/// fn main() {
///     // Define a function for which we want to compute the integral.
///     let func = |x: f64| x.sin(); // f(x) = sin(x)
///
///     // Calculate the integral of the function from 0.0 to π with 1000 subintervals.
///     let result = integral(func, 0.0, std::f64::consts::PI, 1000);
///     println!("The integral from 0.0 to π is approximately: {}", result);
/// }
/// ```
///
/// # Notes
///
/// - The accuracy of the trapezoidal rule approximation improves with a larger number of subintervals `n`.
/// - For very large values of `n`, the computation may become more expensive, so a balance between
///   accuracy and computational cost should be considered.
pub fn integral<F>(func: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (func(a) + func(b));

    for i in 1..n {
        let x = a + i as f64 * h;
        sum += func(x);
    }

    sum * h
}
