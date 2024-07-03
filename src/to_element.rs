use std::ops::Deref;

use leptos::{html::ElementDescriptor, HtmlElement};

use crate::style::{Stylable, Styled};

/// Turns one's type into a leptos element, and hence an SvgElement.
pub trait ToElement<NodeType: ElementDescriptor> {
    fn to_leptos_el(&self) -> HtmlElement<NodeType>;

    fn to_websys_el<WebSys>(&self) -> WebSys
    where
        NodeType: Deref<Target = WebSys>,
        WebSys: Clone,
    {
        self.to_leptos_el().deref().clone()
    }
}

/// Implements for all types that have a default styling implementation
impl<T, N> ToElement<N> for T
where
    T: Stylable,
    T::StyleWith: Default,
    Styled<T>: ToElement<N>,
    N: ElementDescriptor,
{
    fn to_leptos_el(&self) -> HtmlElement<N> {
        self.with_default_style().to_leptos_el()
    }
}
