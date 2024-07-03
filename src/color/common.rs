use super::{AlphaColor, SolidColor};

pub const BLACK: SolidColor = SolidColor::rgb(0, 0, 0);
pub const RED: SolidColor = SolidColor::rgb(255, 0, 0);
pub const GREEN: SolidColor = SolidColor::rgb(0, 255, 0);
pub const BLUE: SolidColor = SolidColor::rgb(0, 0, 255);
pub const WHITE: SolidColor = SolidColor::rgb(255, 255, 255);
pub const GRAY: SolidColor = SolidColor::rgb(128, 128, 128);
pub const YELLOW: SolidColor = SolidColor::rgb(255, 255, 0);
pub const CYAN: SolidColor = SolidColor::rgb(0, 255, 255);
pub const MAGENTA: SolidColor = SolidColor::rgb(255, 0, 255);
pub const TRANSPARENT: AlphaColor = SolidColor::rgb(0, 0, 0).with_alpha_unchecked(0.);
