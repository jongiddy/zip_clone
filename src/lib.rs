//! Zip an iterator to a repeatedly cloned object.
//!
//! Pass an owned object that implements `Clone` to create an iterator that zips
//! the original iterator with clones of the object.
//!
//! One iteration returns the original object, thus using one fewer clones than
//! the otherwise equivalent `iter.zip(repeat_with(|| cloned.clone()))`.
//!
//! Example:
//! ```rust
//! use zip_clone::ZipClone;
//!
//! let s = String::from("Hello");
//! let iter = 0..10;
//! for (i, s) in iter.zip_clone(s) {
//!     assert_eq!(s, String::from("Hello"));
//! }
//! ```

/// Zip an iterator to a repeatedly cloned object.
///
/// One iteration returns the original object, thus using one fewer clones than
/// the otherwise equivalent `iter.zip(repeat_with(|| cloned.clone()))`.
///
/// Example:
/// ```rust
/// use zip_clone::zip_clone;
///
/// let s = String::from("Hello");
/// let iter = 0..10;
/// for (i, s) in zip_clone(iter, s) {
///     assert_eq!(s, String::from("Hello"));
/// }
/// ```
pub fn zip_clone<I, C>(iter: I, cloned: C) -> impl Iterator<Item = (I::Item, C)>
where
    I: Iterator,
    C: Clone,
{
    iter.zip_clone(cloned)
}

/// Trait to zip an iterator to a repeatedly cloned object.
pub trait ZipClone: Iterator + Sized {
    /// Zip an iterator to a repeatedly cloned object.
    ///
    /// One iteration returns the original object, thus using one fewer clones than
    /// the otherwise equivalent `iter.zip(repeat_with(|| cloned.clone()))`.
    ///
    /// Example:
    /// ```rust
    /// use zip_clone::ZipClone;
    ///
    /// let s = String::from("Hello");
    /// let iter = 0..10;
    /// for (i, s) in iter.zip_clone(s) {
    ///     assert_eq!(s, String::from("Hello"));
    /// }
    /// ```
    fn zip_clone<C>(self, cloned: C) -> impl Iterator<Item = (Self::Item, C)>
    where
        C: Clone,
    {
        ZipCloneIter::new(self, cloned)
    }
}

impl<I> ZipClone for I
where
    I: Iterator,
{
    /// Zip an iterator to a repeatedly cloned object.
    ///
    /// One iteration returns the original object, thus using one fewer clones than
    /// the otherwise equivalent `iter.zip(repeat_with(|| cloned.clone()))`.
    ///
    /// Example:
    /// ```rust
    /// use zip_clone::ZipClone;
    ///
    /// let s = String::from("Hello");
    /// let iter = 0..10;
    /// for (i, s) in iter.zip_clone(s) {
    ///     assert_eq!(s, String::from("Hello"));
    /// }
    /// ```
    fn zip_clone<C>(self, cloned: C) -> impl Iterator<Item = (Self::Item, C)>
    where
        C: Clone,
    {
        ZipCloneIter::new(self, cloned)
    }
}

pub struct ZipCloneIter<I, C>
where
    I: Iterator,
{
    iter: std::iter::Peekable<I>,
    cloned: Option<C>,
}

impl<I, C> ZipCloneIter<I, C>
where
    I: Iterator,
    C: Clone,
{
    fn new(iter: I, cloned: C) -> Self {
        Self {
            iter: iter.peekable(),
            cloned: Some(cloned),
        }
    }
}
impl<I, C> Iterator for ZipCloneIter<I, C>
where
    I: Iterator,
    C: Clone,
{
    type Item = (I::Item, C);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.cloned.take(), self.iter.next()) {
            (Some(cloned), Some(item)) => {
                if self.iter.peek().is_some() {
                    self.cloned = Some(cloned.clone());
                }
                Some((item, cloned))
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        match (self.cloned.take(), self.iter.last()) {
            (Some(cloned), Some(item)) => {
                // iterator is fully consumed so no need to replace clone
                Some((item, cloned))
            }
            _ => None,
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match (self.cloned.take(), self.iter.nth(n)) {
            (Some(cloned), Some(item)) => {
                if self.iter.peek().is_some() {
                    self.cloned = Some(cloned.clone());
                }
                Some((item, cloned))
            }
            _ => None,
        }
    }
}

impl<I, C> DoubleEndedIterator for ZipCloneIter<I, C>
where
    I: DoubleEndedIterator,
    C: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match (self.cloned.take(), self.iter.next_back()) {
            (Some(cloned), Some(item)) => {
                if self.iter.peek().is_some() {
                    self.cloned = Some(cloned.clone());
                }
                Some((item, cloned))
            }
            _ => None,
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match (self.cloned.take(), self.iter.nth_back(n)) {
            (Some(cloned), Some(item)) => {
                if self.iter.peek().is_some() {
                    self.cloned = Some(cloned.clone());
                }
                Some((item, cloned))
            }
            _ => None,
        }
    }
}

impl<I, C> ExactSizeIterator for ZipCloneIter<I, C>
where
    I: ExactSizeIterator,
    C: Clone,
{
}

// `ZipCloneIter` is fused because, once the clone is removed and not replaced,
// it will always return `None` for subsequent calls.
impl<I, C> std::iter::FusedIterator for ZipCloneIter<I, C>
where
    I: Iterator,
    C: Clone,
{
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::{zip_clone, ZipClone};

    struct Clonable<'a> {
        count: &'a AtomicU32,
    }
    impl Clone for Clonable<'_> {
        fn clone(&self) -> Self {
            let count = self.count;
            count.fetch_add(1, Ordering::Relaxed);
            Self { count }
        }
    }

    #[test]
    fn test_zip_repeat() {
        let iter = 1..6;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(
            iter.zip(std::iter::repeat_with(|| cloned.clone())).count(),
            5
        );
        assert_eq!(count.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn test_zip_clone() {
        let iter = 1..6;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        // Use map to
        assert_eq!(zip_clone(iter, cloned).map(|_| ()).count(), 5);
        assert_eq!(count.load(Ordering::Relaxed), 4);
    }

    #[test]
    fn test_zip_count() {
        let iter = 1..6;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).count(), 5);
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_zip_last() {
        let iter = 1..6;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).last().unwrap().0, 5);
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_mut_slice() {
        let mut v = vec![1, 2, 3];
        let iter = v.as_mut_slice().iter_mut();
        let s = iter
            .zip_clone(String::new())
            .map(|(i, s)| {
                *i = 1;
                (i, s)
            })
            .collect::<Vec<_>>();
        assert_eq!(&1, s[1].0);
        assert_eq!(&v[1], &1);
    }
}
