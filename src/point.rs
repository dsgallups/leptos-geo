use geo::{CoordNum, Point};
use leptos::{svg::Circle, view, HtmlElement};

use crate::{
    color::{common::BLACK, CssProp, Paint},
    style::{Stylable, Styled},
    ToElement,
};

pub struct PointStyle<T> {
    pub radius: T,
    pub fill: Option<Paint>,
}

impl<T: CoordNum> Default for PointStyle<T> {
    fn default() -> Self {
        Self {
            radius: T::one(),
            fill: Some(Paint::Opaque(BLACK)),
        }
    }
}

impl<T: CoordNum> Stylable for Point<T> {
    type StyleWith = PointStyle<T>;

    fn with_style(&self, style: PointStyle<T>) -> Styled<Point<T>> {
        Styled::new(*self, style)
    }
}

impl<T: CoordNum> ToElement<Circle> for Styled<Point<T>> {
    fn to_leptos_el(&self) -> HtmlElement<Circle> {
        let x = format!("{:?}", self.x());
        let y = format!("{:?}", self.y());
        let radius = format!("{:?}", self.style().radius);
        let fill = self.style().fill.as_ref().map(|f| f.to_css_prop());
        view! {
            <circle cx={x} cy={y} radius={radius} fill={fill}>
            </circle>
        }
    }
}
