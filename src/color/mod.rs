pub mod alpha_color;
use std::str::FromStr;

pub use alpha_color::*;

pub mod solid_color;
pub use solid_color::*;

pub mod paint;
pub use paint::*;

pub mod gradient;
pub use gradient::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod common;

pub type ColorResult<T> = Result<T, ColoringError>;

pub trait Color: FromStr {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum UniformColor {
    Solid(SolidColor),
    Alpha(AlphaColor),
}

impl CssProp for UniformColor {
    fn to_css_prop(&self) -> String {
        use UniformColor::*;
        match self {
            Solid(s) => s.to_css_prop(),
            Alpha(a) => a.to_css_prop(),
        }
    }
}

impl PartialEq for UniformColor {
    /// This will compare an alpha value to an rgb value IF AND ONLY IF
    /// the alpha is 1.0. then, their components will be calculated.
    ///
    /// the reverse of eq DOES not imply ne, and therefore, SolidColor does NOT implement Eq.
    ///
    /// a != b may not equal !(a == b).
    fn eq(&self, other: &Self) -> bool {
        use UniformColor::*;
        match (self, other) {
            // rgba and rgb are effectively the same
            (Solid(o), Alpha(a)) | (Alpha(a), Solid(o)) => {
                let (a_r, a_g, a_b, a_a) = a.as_rgba();
                let (o_r, o_g, o_b) = o.as_rgb();
                //a.a == 1.0 && o.r == a.r && o.g == a.g && o.b == a.b
                a_a == 1.0 && o_r == a_r && o_g == a_g && o_b == a_b
            }
            (Solid(o), Solid(t)) => o == t,
            (Alpha(o), Alpha(t)) => o == t,
        }
    }
}

#[derive(Error, Debug)]
pub enum ColoringError {
    #[error("An error occurred during a color conversion: {0}")]
    CantConvert(String),
    #[error("The property {0} is invalid.")]
    InvalidProperty(String),
}

impl ColoringError {
    pub fn property(s: impl Into<String>) -> Self {
        ColoringError::InvalidProperty(s.into())
    }
    pub fn conversion(s: impl Into<String>) -> Self {
        ColoringError::CantConvert(s.into())
    }
}

pub trait CssProp {
    fn to_css_prop(&self) -> String;
}
