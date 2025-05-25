//! Zip an iterator to a repeatedly cloned value.
//!
//! Pass a value that implements `Clone` to create an iterator that zips
//! the original iterator with clones of the value.
//!
//! One iteration returns the original value, using one fewer clones than
//! `iter.zip(repeat_with(|| cloned.clone()))`.
//!
//! This is useful for loops where a value is cloned for each iteration, but is not
//! used after the iteration.
//!
//! Instead of cloning the `String` 10 times using:
//! ```rust
//! let mut v = vec![String::new(); 10];
//! let hello = String::from("Hello");
//! for elem in v.iter_mut() {
//!     // `hello` cloned 10 times
//!     *elem = hello.clone();
//! }
//! ```
//! clone the `String` 9 times using:
//! ```rust
//! use zip_clone::ZipClone as _;
//!
//! let mut v = vec![String::new(); 10];
//! let hello = String::from("Hello");
//! for (elem, hello) in v.iter_mut().zip_clone(hello) {
//!     // `hello` cloned 9 times, 1 element gets the original `hello`
//!     *elem = hello;
//! }
//! ```
//!
//! This is especially useful when an iterator *commonly* returns a single value, but can return more values, to avoid cloning for the common case:
//! ```rust
//! # use zip_clone::ZipClone as _;
//! # fn get_email_recepients(_: &u32) -> &'static str {"user@example.com"}
//! # let email = 0;
//! let recepients = get_email_recepients(&email); // separated by ,
//! let mut v = vec![];
//! let s = String::from("Sent to ");
//! for (recepient, mut message) in recepients.split(',').zip_clone(s) {
//!     message.push_str(recepient);
//!     v.push(message);
//! }
//! ```
//!
//! `zip_clone` avoids cloning if items are skipped using methods including `last`, `nth` and `skip`.
//! The following code uses the original `String` for the single value produced, avoiding any cloning.
//! ```rust
//! # use zip_clone::ZipClone as _;
//! let hello = String::from("Hello");
//! let _ = (0..10).zip_clone(hello).last();
//! ```
//!
//! For other methods, if possible, it is better to filter the iterator before adding `zip_clone`:
//! ```rust
//! # use zip_clone::ZipClone as _;
//! let mut v = vec![String::new(); 10];
//! let hello = String::from("Hello");
//! for (elem, hello) in v.iter_mut().take(5).zip_clone(hello) {
//!     // `hello` cloned 4 times, 1 element gets the original `hello`
//!     *elem = hello;
//! }
//! ```

/// Zip an iterator to a repeatedly cloned value.
///
/// One iteration returns the original value, thus using one fewer clones than
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
    iter.zip_clone(cloned)
}

/// Trait to zip an iterator to a repeatedly cloned value.
pub trait ZipClone: Iterator + Sized {
    /// Zip an iterator to a repeatedly cloned value.
    ///
    /// One iteration returns the original value, thus using one fewer clones than
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
        ZipCloneIter::new(self, cloned)
    }
}

impl<I> ZipClone for I
where
    I: Iterator,
{
    /// Zip an iterator to a repeatedly cloned value.
    ///
    /// One iteration returns the original value, thus using one fewer clones than
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

    fn respond(&mut self, item: Option<I::Item>) -> Option<<Self as Iterator>::Item> {
        match (item, self.cloned.take()) {
            (Some(item), Some(cloned)) => {
                if self.iter.peek().is_some() {
                    self.cloned = Some(cloned.clone());
                }
                Some((item, cloned))
            }
            _ => None,
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
        let item = self.iter.next();
        self.respond(item)
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
        match (self.iter.last(), self.cloned.take()) {
            (Some(item), Some(cloned)) => {
                // iterator is fully consumed so no need to replace clone
                Some((item, cloned))
            }
            _ => None,
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let item = self.iter.nth(n);
        self.respond(item)
    }
}

impl<I, C> DoubleEndedIterator for ZipCloneIter<I, C>
where
    I: DoubleEndedIterator,
    C: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.iter.next_back();
        self.respond(item)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let item = self.iter.nth_back(n);
        self.respond(item)
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
