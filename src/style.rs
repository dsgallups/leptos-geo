use std::ops::Deref;

/// In theory, implementing this on HtmlElements provides a better way to
/// convert one type into an HtmlElement.
pub trait Stylable: Sized {
    type StyleWith;

    fn with_style(&self, style: Self::StyleWith) -> Styled<Self>
    where
        Self: Sized;

    fn with_default_style(&self) -> Styled<Self>
    where
        Self: Sized,
        Self::StyleWith: Default,
    {
        self.with_style(<Self as Stylable>::StyleWith::default())
    }
}

pub struct Styled<T: Stylable> {
    inner: T,
    style: T::StyleWith,
}

impl<T: Stylable> Styled<T> {
    pub fn new(inner: T, style: T::StyleWith) -> Self {
        Self { inner, style }
    }

    pub fn style(&self) -> &T::StyleWith {
        &self.style
    }
    pub fn style_mut(&mut self) -> &mut T::StyleWith {
        &mut self.style
    }
}

impl<T: Stylable> Deref for Styled<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<&T> for Styled<T>
where
    T: Stylable,
    T::StyleWith: Default,
{
    fn from(value: &T) -> Self {
        value.with_style(<T as Stylable>::StyleWith::default())
    }
}
