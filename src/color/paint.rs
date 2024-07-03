use serde::{Deserialize, Serialize};

use super::{AlphaColor, CssProp, Gradient, SolidColor};

/// Defines a set of all possible ways to paint something.
///
/// Does NOT include patterns, but this should be added. TODO
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Paint {
    Opaque(SolidColor),
    Alpha(AlphaColor),
    Gradient(Gradient),
}

impl CssProp for Paint {
    fn to_css_prop(&self) -> String {
        use Paint::*;
        match self {
            Opaque(o) => o.to_css_prop(),
            Alpha(a) => a.to_css_prop(),
            Gradient(g) => g.to_css_prop(),
        }
    }
}
