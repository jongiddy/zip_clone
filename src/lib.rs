//! Zip an iterator to a repeatedly cloned value.
//! Returns an iterator of 2-tuples containing an iterator item and a clone of the value.
//!
//! ```rust
//! use zip_clone::ZipClone as _;
//!
//! let mut iter = vec![2, 3, 4].into_iter().zip_clone("abc".to_owned());
//! assert_eq!(iter.next(), Some((2, "abc".to_owned())));
//! assert_eq!(iter.next(), Some((3, "abc".to_owned())));
//! assert_eq!(iter.next(), Some((4, "abc".to_owned())));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! One iteration returns the original value, using one fewer clones than
//! `iter.zip(repeat_with(|| cloned.clone()))`.
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
//! # use zip_clone::ZipClone as _;
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
//! let messages = get_email_recepients(&email)
//!     .split(',')
//!     .zip_clone(String::from("Sent to "))
//!     .map(|(recepient, mut message)| {
//!         message.push_str(recepient);
//!         message
//!     })
//!     .collect::<Vec<String>>();
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
/// let mut iter = zip_clone(vec![2, 3, 4].into_iter(), "abc".to_owned());
/// assert_eq!(iter.next(), Some((2, "abc".to_owned())));
/// assert_eq!(iter.next(), Some((3, "abc".to_owned())));
/// assert_eq!(iter.next(), Some((4, "abc".to_owned())));
/// assert_eq!(iter.next(), None);
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
    /// use zip_clone::ZipClone as _;
    ///
    /// let mut iter = vec![2, 3, 4].into_iter().zip_clone("abc".to_owned());
    /// assert_eq!(iter.next(), Some((2, "abc".to_owned())));
    /// assert_eq!(iter.next(), Some((3, "abc".to_owned())));
    /// assert_eq!(iter.next(), Some((4, "abc".to_owned())));
    /// assert_eq!(iter.next(), None);
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
    /// use zip_clone::ZipClone as _;
    ///
    /// let mut iter = vec![2, 3, 4].into_iter().zip_clone("abc".to_owned());
    /// assert_eq!(iter.next(), Some((2, "abc".to_owned())));
    /// assert_eq!(iter.next(), Some((3, "abc".to_owned())));
    /// assert_eq!(iter.next(), Some((4, "abc".to_owned())));
    /// assert_eq!(iter.next(), None);
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

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        match (self.iter.next(), self.cloned.take()) {
            (Some(item), Some(cloned)) => {
                let mut tuple = (item, cloned);
                while !predicate(&tuple) {
                    tuple.0 = self.iter.next()?;
                }
                if self.iter.peek().is_some() {
                    self.cloned = Some(tuple.1.clone());
                }
                Some(tuple)
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
        let item = self.iter.next_back();
        self.respond(item)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let item = self.iter.nth_back(n);
        self.respond(item)
    }

    fn rfind<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        match (self.iter.next_back(), self.cloned.take()) {
            (Some(item), Some(cloned)) => {
                let mut tuple = (item, cloned);
                while !predicate(&tuple) {
                    tuple.0 = self.iter.next_back()?;
                }
                if self.iter.peek().is_some() {
                    self.cloned = Some(tuple.1.clone());
                }
                Some(tuple)
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
        let iter = 1..=5;
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
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        // Use map to avoid the more efficient `count` method on `ZipClone`
        assert_eq!(zip_clone(iter, cloned).map(|_| ()).count(), 5);
        assert_eq!(count.load(Ordering::Relaxed), 4);
    }

    #[test]
    fn test_zip_count() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).count(), 5);
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_zip_last() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).last().unwrap().0, 5);
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_zip_find_first() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).find(|_| true).unwrap().0, 1);
        // return one clone with result, keep one clone for further iteration
        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_zip_find_mid() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).find(|x| x.0 == 3).unwrap().0, 3);
        // return one clone with result, keep one clone for further iteration
        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_zip_find_last() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).find(|x| x.0 == 5).unwrap().0, 5);
        // return one clone with result, no other clone needed
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    #[allow(clippy::search_is_some)]
    fn test_zip_find_not_found() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert!(zip_clone(iter, cloned).find(|_| false).is_none());
        // clone not required
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_zip_rfind_first() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).rfind(|_| true).unwrap().0, 5);
        // return one clone with result, keep one clone for further iteration
        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_zip_rfind_mid() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).rfind(|x| x.0 == 3).unwrap().0, 3);
        // return one clone with result, keep one clone for further iteration
        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_zip_rfind_last() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(zip_clone(iter, cloned).rfind(|x| x.0 == 1).unwrap().0, 1);
        // return one clone with result, no other clone needed
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    #[allow(clippy::search_is_some)]
    fn test_zip_rfind_not_found() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert!(zip_clone(iter, cloned).rfind(|_| false).is_none());
        // clone not required
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_by_ref() {
        let iter = 1..=5;
        let count = AtomicU32::new(0);
        let cloned = Clonable { count: &count };
        assert_eq!(iter.zip_clone(cloned).by_ref().count(), 5);
        assert_eq!(count.load(Ordering::Relaxed), 4);
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
