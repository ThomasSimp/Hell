use std::f64::consts::PI;

/// Computes the sine of an angle given in radians.
pub fn sine(angle_rad: f64) -> f64 {
    angle_rad.sin()
}

/// Computes the cosine of an angle given in radians.
pub fn cosine(angle_rad: f64) -> f64 {
    angle_rad.cos()
}

/// Computes the tangent of an angle given in radians.
pub fn tangent(angle_rad: f64) -> f64 {
    angle_rad.tan()
}

/// Computes the inverse sine (arcsine) of a value. Returns the angle in radians.
pub fn arcsine(value: f64) -> Option<f64> {
    if value < -1.0 || value > 1.0 {
        None // arcsine is only defined for values in the range [-1, 1]
    } else {
        Some(value.asin())
    }
}

/// Computes the inverse cosine (arccosine) of a value. Returns the angle in radians.
pub fn arccosine(value: f64) -> Option<f64> {
    if value < -1.0 || value > 1.0 {
        None // arccosine is only defined for values in the range [-1, 1]
    } else {
        Some(value.acos())
    }
}

/// Computes the inverse tangent (arctangent) of a value. Returns the angle in radians.
pub fn arctangent(value: f64) -> f64 {
    value.atan()
}

/// Converts an angle from radians to degrees.
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

/// Converts an angle from degrees to radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}
