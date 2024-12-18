use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::error::AdventError;

// Most common use-case for a PointT is unsigned
pub type Point = PointT<usize>;

// Signed version of a PointT
pub type PointSig = PointT<isize>;

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PointT<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Debug for PointT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T: Display> Display for PointT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> From<(T, T)> for PointT<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T: Copy> From<&(T, T)> for PointT<T> {
    fn from(value: &(T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<PointT<T>> for (T, T) {
    fn from(value: PointT<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: FromStr> TryFrom<&str> for PointT<T>
where
    AdventError: From<<T as FromStr>::Err>,
{
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (lhs, rhs) = value
            .split_once(',')
            .ok_or(AdventError::SplitOnce(value.to_string(), ','.to_string()))?;

        Ok(PointT {
            x: lhs.parse::<T>()?,
            y: rhs.parse::<T>()?,
        })
    }
}
