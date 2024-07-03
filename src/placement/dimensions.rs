use serde::{Deserialize, Serialize};

/// Dimensions of a rectangle-like object
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Dimensions<Unit = f64> {
    /// Width of the object
    pub width: Unit,
    /// Height of the object
    pub height: Unit,
}
