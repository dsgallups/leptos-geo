use serde::{Deserialize, Serialize};

use super::Vec2;

/// Defines a rotation whose 0 angle points to the east.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rotation(f64);

impl Rotation {
    /// No rotation
    pub const fn zero() -> Rotation {
        Rotation(0.0)
    }

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub const fn flip() -> Rotation {
        Rotation(std::f64::consts::PI)
    }

    pub const fn quarter() -> Rotation {
        Rotation(std::f64::consts::FRAC_PI_2)
    }

    /// Create a new rotation from radians
    pub fn radians(radians: f64) -> Rotation {
        Rotation(radians)
    }
    /// Create a new rotation from degrees
    pub fn degrees(degrees: f64) -> Rotation {
        Rotation(degrees.to_radians())
    }

    /// Create a new rotation from a vector, based on the angle between the vector and the x-axis
    pub fn from_vec(v: Vec2) -> Rotation {
        Rotation(v.y.atan2(v.x))
    }

    /// Get the rotation in radians
    pub fn as_radians(&self) -> f64 {
        self.0
    }

    /// Get the rotation in degrees
    pub fn as_degrees(&self) -> f64 {
        self.0.to_degrees()
    }
}
