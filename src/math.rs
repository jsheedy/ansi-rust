use std::{f64};

pub fn sin(x: f64) -> f64 {
    // sin of normalized value, normalized
    ((x * 2.0 * f64::consts::PI).sin()+1.0) / 2.0
}

pub fn cos(x: f64) -> f64 {
    // cos of normalized value, normalized
    ((x * 2.0 * f64::consts::PI).cos()+1.0) / 2.0
}
