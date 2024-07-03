use serde::{Deserialize, Serialize};

use crate::placement::{Rotation, Vec2};

use super::{CssProp, UniformColor};

/// Defines all possible ways to draw a graident.
///
/// Does not include Conic or curve-based gradients. TODO
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Gradient {
    Linear {
        rot: Rotation,
        colors: Vec<(UniformColor, f64)>,
    },
    Radial {
        center: Vec2,
        colors: Vec<(UniformColor, f64)>,
    },
}

impl Gradient {
    /// Create a new linear gradient
    pub fn linear(rot: Rotation, colors: Vec<(UniformColor, f64)>) -> Self {
        Self::Linear { rot, colors }
    }

    /// Create a new radial gradient
    pub fn radial(center: Vec2, colors: Vec<(UniformColor, f64)>) -> Self {
        Self::Radial { center, colors }
    }
}

impl CssProp for Gradient {
    fn to_css_prop(&self) -> String {
        use Gradient::*;
        match self {
            Linear { rot, colors } => {
                if colors.is_empty() {
                    return "".to_string();
                }

                let mut color_list = String::new();

                for (i, (color, percentage)) in colors.iter().enumerate() {
                    let percentage = (percentage * 100.0).round() as i32;
                    if i == colors.len() - 1 {
                        color_list.push_str(&format!("{} {}%", color.to_css_prop(), percentage));
                    } else {
                        color_list.push_str(&format!("{} {}%,", color.to_css_prop(), percentage));
                    }
                }

                format!(
                    "linear-gradient({}deg, {})",
                    rot.as_degrees().round() as u16,
                    color_list
                )
            }
            Radial {
                center: _,
                colors: _,
            } => {
                //todo
                todo!()
                //todo
            }
        }
    }
}
