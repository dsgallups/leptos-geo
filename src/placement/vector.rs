use std::fmt;

use serde::{Deserialize, Serialize};

/// A 2D vector that typically defines a position or direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec2<T = f64> {
    /// Coordinate on the horizontal plane
    pub x: T,
    /// Coordinate on the vertical plane
    pub y: T,
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Vec2<T> {
    /// Create a new Vec2
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2 {
    /// Create a new Vec2 at (0, 0)
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Get the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Normalize the vector
    pub fn normalize(&self) -> Vec2 {
        let magnitude = self.magnitude();
        Vec2 {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    /// Get the dot product of two vectors
    pub fn dot(&self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
    /// Get the cross product of two vectors
    pub fn cross(&self, other: Vec2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn negate(&self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
