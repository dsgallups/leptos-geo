use serde::{Deserialize, Serialize};

use super::{AlphaColor, ColorResult, ColoringError, CssProp};

/// A simple RGB color (0-255)
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SolidColor {
    /// red
    r: u8,
    /// green
    g: u8,
    /// blue
    b: u8,
}

impl SolidColor {
    /// Create a new RGB color
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn with_alpha(self, a: f64) -> ColorResult<AlphaColor> {
        AlphaColor::rgba(self.r, self.g, self.b, a)
            .map_err(|e| ColoringError::conversion(e.to_string()))
    }

    pub fn as_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Convert the color to a RGBA color. Does not check that the alpha is between 0. and 1.
    pub(crate) const fn with_alpha_unchecked(self, a: f64) -> AlphaColor {
        AlphaColor::rgba_unchecked(self.r, self.g, self.b, a)
    }

    /// Create a new RGB color from a hex string. Returns None if the hex string is not RGB valid.
    pub fn from_hex<Hex: AsRef<str>>(hex: Hex) -> Option<Self> {
        let hex = hex.as_ref();

        let hex = hex.trim_start_matches('#');
        let mut chars = hex.chars();
        let r = TryInto::<u8>::try_into(chars.next()?.to_digit(16)?).ok()?;
        let r = r * 16 + (TryInto::<u8>::try_into(chars.next()?.to_digit(16)?).ok()?);
        let g = TryInto::<u8>::try_into(chars.next()?.to_digit(16)?).ok()?;
        let g = g * 16 + (TryInto::<u8>::try_into(chars.next()?.to_digit(16)?).ok()?);
        let b = TryInto::<u8>::try_into(chars.next()?.to_digit(16)?).ok()?;
        let b = b * 16 + (TryInto::<u8>::try_into(chars.next()?.to_digit(16)?).ok()?);
        Some(Self { r, g, b })
    }

    /// rgb({r}, {g}, {b})
    pub fn from_color_string(str: impl AsRef<str>) -> Option<Self> {
        let str = str.as_ref();
        let open_br = str.find('(')?;

        let close_br = str.find(')')?;

        let mut parts = str[open_br + 1..close_br].split(',');

        let r = parts.next()?.parse().ok()?;
        let g = parts.next()?.parse().ok()?;
        let b = parts.next()?.parse().ok()?;

        Some(Self { r, g, b })
    }
}

impl CssProp for SolidColor {
    /// rgb({r}, {g}, {b})
    fn to_css_prop(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
}
