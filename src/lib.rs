use std::ops::{Add, Range, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span<T> {
    start: T,
    length: T,
}
impl<T> Span<T> {
    #[inline]
    pub fn new(start: T, length: T) -> Self {
        Self { start, length }
    }

    #[inline]
    pub fn from_points(a: T, b: T) -> Self
    where
        T: Copy + Ord + Sub<T, Output = T>,
    {
        let low = a.min(b);
        let hi = a.max(b);
        let len = hi - low;
        Self::new(low, len)
    }

    #[inline]
    pub fn start(self) -> T
    where
        T: Copy,
    {
        self.start
    }
    #[inline]
    pub fn length(self) -> T
    where
        T: Copy,
    {
        self.length
    }
    #[inline]
    pub fn end(self) -> T
    where
        T: Copy + Add<T, Output = T>,
    {
        self.start + self.length
    }

    #[inline]
    pub fn merge(a: Self, b: Self) -> Self
    where
        T: Copy + Ord + Add<T, Output = T> + Sub<T, Output = T>,
    {
        let start = a.start.min(b.start);
        let end = a.end().max(b.end());
        Self::from_points(start, end)
    }

    #[inline]
    pub fn range(self) -> Range<T>
    where
        T: Copy + Add<T, Output = T>,
    {
        let end = self.end();
        self.start..end
    }
}
