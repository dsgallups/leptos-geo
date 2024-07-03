use geo::CoordFloat;

pub trait ViewBoxed<T: CoordFloat> {
    fn viewbox(&self) -> ViewBox<T>;
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ViewBox<T> {
    pub min_x: Option<T>,
    pub min_y: Option<T>,
    pub max_x: Option<T>,
    pub max_y: Option<T>,
}

impl<T: CoordFloat> ViewBox<T> {
    pub fn new(min_x: T, min_y: T, max_x: T, max_y: T) -> Self {
        Self {
            min_x: Some(min_x),
            min_y: Some(min_y),
            max_x: Some(max_x),
            max_y: Some(max_y),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            min_x: Self::min_option(self.min_x, other.min_x),
            min_y: Self::min_option(self.min_y, other.min_y),
            max_x: Self::max_option(self.max_x, other.max_x),
            max_y: Self::max_option(self.max_y, other.max_y),
        }
    }

    pub fn min_x(&self) -> T {
        self.min_x.unwrap_or(T::zero())
    }

    pub fn min_y(&self) -> T {
        self.min_y.unwrap_or(T::zero())
    }

    pub fn max_x(&self) -> T {
        self.max_x.unwrap_or(T::zero())
    }

    pub fn max_y(&self) -> T {
        self.max_y.unwrap_or(T::zero())
    }

    pub fn width(&self) -> T {
        (self.min_x() - self.max_x()).abs()
    }

    pub fn height(&self) -> T {
        (self.min_y() - self.max_y()).abs()
    }

    fn min_option(a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn max_option(a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn with_margin(mut self, margin: T) -> Self {
        self.min_x = self.min_x.map(|x| x - margin);
        self.min_y = self.min_y.map(|y| y - margin);
        self.max_x = self.max_x.map(|x| x + margin);
        self.max_y = self.max_y.map(|y| y + margin);
        self
    }
}
