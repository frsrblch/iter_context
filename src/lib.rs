#![forbid(missing_docs)]

//! An iterator for use with a [Struct-of-Arrays] data layout, where data is associated by index
//! within the arrays of the struct.
//!
//! Traits and related types for defining and using [ContextualIterator].
//! Implementors of `ContextualIterator` shall uphold that any `ContextualIterator`
//! of the same `Context` have the same number of items and that the items are related by
//! their position in the iterator.
//!
//! [Struct-of-Arrays]: https://en.wikipedia.org/wiki/AoS_and_SoA#Structure_of_arrays
pub use crate::iter::{Iter, IterMut};
pub use crate::map::Map;
pub use crate::zip::Zip;

mod iter;
mod map;
mod zip;

/// An Iterator over a given `Context`.
///
/// Types implementing `ContextualIterator` with the same `Context` should all be of the same length and
/// aligned by their position in the iterator, like the values in different columns of a table.
///
/// The methods on `ContextualIterator` cannot change the positional alignment of the iterator,
/// so `ContextualIterator` can be zipped together and have their values mapped while remaining a
/// `ContextualIterator` over the same `Context`.
///
/// Types implementing `ContextualIterator` should not implement `Iterator` directly because
/// many of the `Iterator` functions, such as [`std::iter::Iterator::filter`], can cause iterator misalignment.
/// `ContextualIterator` all implement `IntoIterator`, and can be converted into `Iterator` when needed.
///
/// Use [`IntoIterator::into_iter`]  or [`ContextualIterator::for_each`] method after all
/// desired data has been mapped and zipped together.
///
/// [`for_each`]: trait.ContextualIterator.html#method.for_each
pub trait ContextualIterator: IntoIterator + Sized {
    /// A `ContextualIterator` can only be zipped to another `ContextualIterator` with the same `Context`.
    type Context;

    /// Zip together two `ContextualIterator` with the same Context to return a single `ContextualIterator`
    /// with the same Context.
    ///
    /// Analogous to [std::iter::Iterator::zip].
    fn zip<U>(self, rhs: U) -> Zip<Self::Context, Self, U>
    where
        U: ContextualIterator<Context = Self::Context>,
    {
        Zip::new(self, rhs)
    }

    /// Map the values from a `ContextualIterator` using the given closure to return a `ContextualIterator` of
    /// the mapped values.
    ///
    /// Analogous to [std::iter::Iterator::map].
    fn map<B, F>(self, f: F) -> Map<Self::Context, Self, F>
    where
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    /// Consume the `ContextualIterator` and call the closure on each element.
    ///
    /// Analogous to [std::iter::Iterator::for_each].
    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        self.into_iter().for_each(f);
    }

    /// Collects the values from a `ContextualIterator`
    /// into a collection that implements [FromContextualIterator].
    ///
    /// Analogous to [std::iter::Iterator::collect].
    #[must_use]
    fn collect<B>(self) -> B
    where
        B: FromContextualIterator<Self::Item, Context = Self::Context>,
    {
        FromContextualIterator::from_iter(self)
    }
}

/// Collection from a [ContextualIterator].
///
/// Analogous to [std::iter::FromIterator].
pub trait FromContextualIterator<Item> {
    /// The context of the collection that will be collected from the iterator.
    type Context;
    /// Collects the values from the given [ContextualIterator] and returns a collection
    /// with the same Context.
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: ContextualIterator<Context = Self::Context, Item = Item>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles() {
        let mut a = [1u32, 2, 3];
        let b = [2u32, 3, 5];

        IterMut::<(), u32>::new(a.iter_mut())
            .zip(Iter::<(), u32>::new(b.iter()))
            .for_each(|(a, b)| {
                *a += *b;
            });

        assert_eq!(a, [3, 5, 8]);
    }

    // #[test]
    // fn does_not_compile() {
    //     let mut a = [1u32, 2, 3];
    //     let b = [2u32, 3, 5];
    //
    //     IterMut::<u8, u32>::new(a.iter_mut())
    //         .zip(Iter::<(), u32>::new(b.iter()))
    //         .for_each(|(a, b)| {
    //             *a += *b;
    //         });
    // }
}
