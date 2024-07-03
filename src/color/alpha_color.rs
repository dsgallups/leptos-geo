use serde::{Deserialize, Serialize};

use super::{ColorResult, ColoringError, CssProp};

/// A simple RGBA color (0-255, 0.0-1.0)
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct AlphaColor {
    /// red
    r: u8,
    /// green
    g: u8,
    /// blue
    b: u8,
    /// alpha (opacity betwen 0. and 1.)
    a: f64,
}

impl AlphaColor {
    /// Create a new RGBA color without checking that the alpha is between 0. and 1.
    pub(crate) const fn rgba_unchecked(r: u8, g: u8, b: u8, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> ColorResult<Self> {
        if !(0. ..=1.).contains(&a) {
            return Err(ColoringError::InvalidProperty(
                "Alpha value must be between 0. and 1.".to_string(),
            ));
        }
        Ok(Self { r, g, b, a })
    }

    /// Returns the color as an RGBA tuple, in that order.
    pub fn as_rgba(&self) -> (u8, u8, u8, f64) {
        (self.r, self.g, self.b, self.a)
    }
}

impl CssProp for AlphaColor {
    /// rgba({r}, {g}, {b}, {a})
    fn to_css_prop(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}
