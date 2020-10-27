pub use crate::iter::{Iter, IterMut};
use crate::map::Map;
pub use crate::zip::Zip;

mod iter;
mod map;
mod zip;

/// An iterator for use with a Struct-of-Arrays data layout, where data is associated by index
/// within the arrays of the struct.
///
/// Types implementing `TypedIterator` with the same `Context` should all be of the same length and
/// aligned by index, like columns in a table. Types implementing `TypedIterator` should not implement
/// `Iterator` because many of the `Iterator` functions can cause data misalignment.
///
/// Use `IntoIterator::into_iter()` method or `TypedIterator`'s [`for_each`] method after all
/// desired data has been zipped together.
///
/// [`for_each`]: trait.TypedIterator.html#method.for_each
pub trait TypedIterator: IntoIterator + Sized {
    /// A `TypedIterator` can only be zipped to another `TypedIterator` with the same `Context`.
    type Context;

    /// Zip together two `TypedIterator` with the same Context to return a single `TypedIterator`
    /// with that same Context.
    fn zip<U: TypedIterator<Context = Self::Context>>(self, rhs: U) -> Zip<Self::Context, Self, U> {
        Zip::new(self, rhs)
    }

    /// Map the values from a `TypedIterator` using the given closure to return a `TypedIterator` of 
    /// the mapped values.
    fn map<B, F: FnMut(Self::Item) -> B>(self, f: F) -> Map<Self::Context, Self, F> {
        Map::new(self, f)
    }

    /// Consume the `TypedIterator` and call the closure on each element.
    fn for_each<F: FnMut(Self::Item)>(self, f: F) {
        self.into_iter().for_each(f);
    }
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
