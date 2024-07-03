use geo::{CoordFloat, CoordNum, Point};
use leptos::{
    svg::{Circle, Svg},
    view, HtmlElement,
};

use crate::{
    color::{common::BLACK, CssProp, Paint},
    style::{Stylable, Styled},
    viewbox::{ViewBox, ViewBoxed},
    ToElement,
};

pub struct PointStyle<T> {
    pub radius: T,
    pub fill: Option<Paint>,
    pub stroke_width: T,
}

impl<T: CoordNum> Default for PointStyle<T> {
    fn default() -> Self {
        Self {
            radius: T::one(),
            fill: Some(Paint::Opaque(BLACK)),
            stroke_width: T::zero(),
        }
    }
}

impl<T: CoordNum> Stylable for Point<T> {
    type StyleWith = PointStyle<T>;

    fn with_style(&self, style: PointStyle<T>) -> Styled<Point<T>> {
        Styled::new(*self, style)
    }
}

impl<T: CoordFloat> ViewBoxed<T> for Styled<Point<T>> {
    fn viewbox(&self) -> ViewBox<T> {
        let radius = self.style().radius + self.style().stroke_width;
        ViewBox::new(
            self.x() - radius,
            self.y() - radius,
            self.x() + radius,
            self.y() + radius,
        )
    }
}

impl<T: CoordFloat> ToElement<Svg> for Styled<Point<T>> {
    fn to_leptos_el(&self) -> HtmlElement<Svg> {
        let circle = <Self as ToElement<Circle>>::to_leptos_el(self);
        let vb = self.viewbox();
        let viewbox = format!(
            "{:?} {:?} {:?} {:?}",
            vb.min_x(),
            vb.min_y(),
            vb.width(),
            vb.height()
        );

        view! {
            <svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewbox={viewbox}>
                {circle}
            </svg>
        }
    }
}

impl<T: CoordFloat> ToElement<Circle> for Styled<Point<T>> {
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
