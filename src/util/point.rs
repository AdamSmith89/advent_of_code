#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point(pub usize, pub usize);

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<&(usize, usize)> for Point {
    fn from(value: &(usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}