/// Solves a quadratic equation of the form `ax^2 + bx + c = 0`.
///
/// This function computes the roots of a quadratic equation using the quadratic formula:
/// 
/// \[
/// x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
/// \]
///
/// # Arguments
///
/// * `a` - The coefficient of the quadratic term (`x^2`). Must be a non-zero `f64` value 
///   because division by zero would otherwise occur.
/// * `b` - The coefficient of the linear term (`x`).
/// * `c` - The constant term.
///
/// # Returns
///
/// * `Option<(f64, f64)>` - Returns `Some((root1, root2))` where `root1` and `root2` are the 
///   two real roots of the quadratic equation. If the discriminant is negative, indicating that 
///   the roots are complex (not real), the function returns `None`.
///
/// # Examples
///
/// ```rust
/// assert_eq!(solve_quadratic(1.0, -3.0, 2.0), Some((2.0, 1.0)));
/// assert_eq!(solve_quadratic(1.0, 2.0, 5.0), None); // No real roots
/// ```
///
/// # Panics
///
/// The function does not explicitly panic, but providing `a = 0.0` will result in a division by 
/// zero, which will cause a runtime panic. It is assumed that the user will provide valid input 
/// where `a` is non-zero.
///
/// # Performance
///
/// The function is efficient and operates in constant time `O(1)` since it performs a fixed number 
/// of arithmetic operations regardless of the input size.
///
/// # Usage
///
/// This function is useful in mathematical computations where finding the roots of a quadratic 
/// equation is necessary, such as in physics, engineering, and computer graphics. It can be used 
/// to determine intersections, optimize functions, or model various phenomena.
///
/// # Limitations
///
/// The function assumes that `a`, `b`, and `c` are finite real numbers (`f64`). The accuracy of the 
/// results may be limited by the precision of floating-point arithmetic, especially for very small 
/// or very large values of `a`, `b`, or `c`. Additionally, the function does not handle the case 
/// where `a = 0.0`, which would reduce the equation to a linear equation (`bx + c = 0`).
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        return None; // No real roots
    }
    
    let sqrt_discriminant = discriminant.sqrt();
    let root1 = (-b + sqrt_discriminant) / (2.0 * a);
    let root2 = (-b - sqrt_discriminant) / (2.0 * a);
    
    Some((root1, root2))
}
