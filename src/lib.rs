//! Zip an iterator to a repeately cloned object.
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

/// Zip an iterator to a repeately cloned object.
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
pub fn zip_clone<I, C>(iter: I, cloned: C) -> ZipCloneIter<I, C>
where
    I: Iterator,
    C: Clone,
{
    ZipCloneIter {
        iter: iter.peekable(),
        cloned: Some(cloned),
    }
}

/// Trait to zip an iterator to a repeately cloned object.
pub trait ZipClone: Iterator + Sized {
    fn zip_clone<C>(self, cloned: C) -> ZipCloneIter<Self, C>
    where
        C: Clone;
}

impl<I> ZipClone for I
where
    I: Iterator,
{
    /// Zip an iterator to a repeately cloned object.
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
    fn zip_clone<C>(self, cloned: C) -> ZipCloneIter<Self, C>
    where
        C: Clone,
    {
        zip_clone(self, cloned)
    }
}

pub struct ZipCloneIter<I, C>
where
    I: Iterator,
{
    iter: std::iter::Peekable<I>,
    cloned: Option<C>,
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
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::zip_clone;

    #[test]
    fn test_zip_clone() {
        struct Clonable<'a> {
            count: &'a AtomicU32,
        }
        impl<'a> Clone for Clonable<'a> {
            fn clone(&self) -> Self {
                let count = self.count;
                count.fetch_add(1, Ordering::Relaxed);
                Self { count }
            }
        }
        let iter = 1..6;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).count(), 5);
        assert_eq!(count.load(Ordering::Relaxed), 4);
    }

    #[test]
    fn test_zip_repeat() {
        struct Clonable<'a> {
            count: &'a AtomicU32,
        }
        impl<'a> Clone for Clonable<'a> {
            fn clone(&self) -> Self {
                let count = self.count;
                count.fetch_add(1, Ordering::Relaxed);
                Self { count }
            }
        }
        let iter = 1..6;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(
            iter.zip(std::iter::repeat_with(|| cloned.clone())).count(),
            5
        );
        assert_eq!(count.load(Ordering::Relaxed), 5);
    }
}
