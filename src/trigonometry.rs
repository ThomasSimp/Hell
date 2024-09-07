use std::f64::consts::PI;

/// Computes the sine of an angle provided in radians.
/// 
/// # Arguments
///
/// * `angle_rad` - A `f64` representing the angle in radians for which the sine will be calculated.
///
/// # Returns
///
/// * A `f64` representing the sine of the given angle. 
///   The sine function returns a value between -1 and 1 inclusive, and it corresponds to the 
///   y-coordinate of the point on the unit circle that the angle (in radians) subtends.
///
/// # Example
///
/// ```
/// let angle = std::f64::consts::PI / 2.0; // 90 degrees
/// let sine_value = sine(angle);
/// assert_eq!(sine_value, 1.0);
/// ```
///
/// # Notes
/// 
/// The sine function is periodic with a period of 2π, meaning `sin(θ) = sin(θ + 2πk)` for any integer `k`.
pub fn sine(angle_rad: f64) -> f64 {
    angle_rad.sin()
}

/// Computes the cosine of an angle provided in radians.
///
/// # Arguments
///
/// * `angle_rad` - A `f64` representing the angle in radians for which the cosine will be calculated.
///
/// # Returns
///
/// * A `f64` representing the cosine of the given angle. 
///   The cosine function returns a value between -1 and 1 inclusive, and it corresponds to the 
///   x-coordinate of the point on the unit circle that the angle (in radians) subtends.
///
/// # Example
///
/// ```
/// let angle = std::f64::consts::PI; // 180 degrees
/// let cosine_value = cosine(angle);
/// assert_eq!(cosine_value, -1.0);
/// ```
///
/// # Notes
/// 
/// The cosine function is periodic with a period of 2π, meaning `cos(θ) = cos(θ + 2πk)` for any integer `k`.
pub fn cosine(angle_rad: f64) -> f64 {
    angle_rad.cos()
}

/// Computes the tangent of an angle provided in radians.
///
/// # Arguments
///
/// * `angle_rad` - A `f64` representing the angle in radians for which the tangent will be calculated.
///
/// # Returns
///
/// * A `f64` representing the tangent of the given angle. 
///   The tangent function returns values that range from negative infinity to positive infinity.
///   It is the ratio of the sine of the angle to its cosine: `tan(θ) = sin(θ) / cos(θ)`.
///
/// # Example
///
/// ```
/// let angle = std::f64::consts::PI / 4.0; // 45 degrees
/// let tangent_value = tangent(angle);
/// assert_eq!(tangent_value, 1.0);
/// ```
///
/// # Notes
/// 
/// The tangent function has vertical asymptotes at odd multiples of π/2 (e.g., π/2, 3π/2, etc.), 
/// where the function is undefined and tends toward infinity or negative infinity.
pub fn tangent(angle_rad: f64) -> f64 {
    angle_rad.tan()
}

/// Computes the inverse sine (arcsine) of a value and returns the corresponding angle in radians.
///
/// # Arguments
///
/// * `value` - A `f64` representing the sine value for which the angle will be calculated. 
///   The value must lie within the range [-1, 1], inclusive.
///
/// # Returns
///
/// * An `Option<f64>` where `Some(angle)` is the result in radians, and `None` is returned if the input
///   is outside the valid range of [-1, 1]. The result angle will be in the range of [-π/2, π/2].
///
/// # Example
///
/// ```
/// let value = 0.5;
/// if let Some(angle) = arcsine(value) {
///     assert!((angle - std::f64::consts::PI / 6.0).abs() < 1e-10); // 30 degrees in radians
/// }
/// ```
///
/// # Notes
///
/// The arcsine function is the inverse of the sine function. It returns an angle such that `sin(angle) = value`.
pub fn arcsine(value: f64) -> Option<f64> {
    if value < -1.0 || value > 1.0 {
        None // arcsine is only defined for values in the range [-1, 1]
    } else {
        Some(value.asin())
    }
}

/// Computes the inverse cosine (arccosine) of a value and returns the corresponding angle in radians.
///
/// # Arguments
///
/// * `value` - A `f64` representing the cosine value for which the angle will be calculated. 
///   The value must lie within the range [-1, 1], inclusive.
///
/// # Returns
///
/// * An `Option<f64>` where `Some(angle)` is the result in radians, and `None` is returned if the input
///   is outside the valid range of [-1, 1]. The result angle will be in the range of [0, π].
///
/// # Example
///
/// ```
/// let value = 1.0;
/// if let Some(angle) = arccosine(value) {
///     assert_eq!(angle, 0.0); // arccosine of 1.0 is 0 radians (0 degrees)
/// }
/// ```
///
/// # Notes
///
/// The arccosine function is the inverse of the cosine function. It returns an angle such that `cos(angle) = value`.
pub fn arccosine(value: f64) -> Option<f64> {
    if value < -1.0 || value > 1.0 {
        None // arccosine is only defined for values in the range [-1, 1]
    } else {
        Some(value.acos())
    }
}

/// Computes the inverse tangent (arctangent) of a value and returns the corresponding angle in radians.
///
/// # Arguments
///
/// * `value` - A `f64` representing the tangent value for which the angle will be calculated. 
///
/// # Returns
///
/// * A `f64` representing the angle in radians, in the range of [-π/2, π/2].
///
/// # Example
///
/// ```
/// let value = 1.0;
/// let angle = arctangent(value);
/// assert_eq!(angle, std::f64::consts::PI / 4.0); // arctangent of 1.0 is π/4 radians (45 degrees)
/// ```
///
/// # Notes
///
/// The arctangent function is the inverse of the tangent function. It returns an angle such that `tan(angle) = value`.
pub fn arctangent(value: f64) -> f64 {
    value.atan()
}

/// Converts an angle from radians to degrees.
///
/// # Arguments
///
/// * `radians` - A `f64` representing the angle in radians.
///
/// # Returns
///
/// * A `f64` representing the angle in degrees. The result is calculated by multiplying the 
///   input value by `180 / π`.
///
/// # Example
///
/// ```
/// let angle_rad = std::f64::consts::PI;
/// let angle_deg = radians_to_degrees(angle_rad);
/// assert_eq!(angle_deg, 180.0);
/// ```
///
/// # Notes
///
/// This function is useful when working with systems or environments where angles are commonly 
/// expressed in degrees rather than radians.
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

/// Converts an angle from degrees to radians.
///
/// # Arguments
///
/// * `degrees` - A `f64` representing the angle in degrees.
///
/// # Returns
///
/// * A `f64` representing the angle in radians. The result is calculated by multiplying the 
///   input value by `π / 180`.
///
/// # Example
///
/// ```
/// let angle_deg = 180.0;
/// let angle_rad = degrees_to_radians(angle_deg);
/// assert_eq!(angle_rad, std::f64::consts::PI);
/// ```
///
/// # Notes
///
/// This function is useful when working with trigonometric functions, which typically expect
/// angles in radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}
