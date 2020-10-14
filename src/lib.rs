pub use crate::iter::{Iter, IterMut};
pub use crate::tuple::Tuple;
pub use crate::zip::Zip;
use std::marker::PhantomData;

mod iter;
mod tuple;
mod zip;

pub trait IterOver: IntoIterator + Sized {
    type Type;

    fn zip<U: IterOver<Type = Self::Type>>(self, rhs: U) -> Zip<Self::Type, Self, U> {
        Zip::new(self, rhs)
    }

    fn for_each<F: FnMut(Self::Item)>(self, f: F) {
        self.into_iter().for_each(f);
    }
}

pub trait ForEachInner
where
    Self: IntoIterator + Sized,
    <Self as IntoIterator>::Item: IntoIterator,
{
    fn for_each_inner<F: FnMut(<Self::Item as IntoIterator>::Item)>(self, mut f: F) {
        self.into_iter().for_each(|outer| {
            outer.into_iter().for_each(|t| f(t));
        })
    }
}

impl<T> ForEachInner for T
where
    T: IntoIterator,
    <T as IntoIterator>::Item: IntoIterator,
{
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
            .for_each(|Tuple(a, b)| {
                *a += *b;
            });
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

    #[test]
    fn for_each_inner() {
        let a = vec![vec![0u32, 1], vec![2u32, 3]];
        let b = vec![vec![1u32, 2], vec![3u32, 5]];
        let c = vec![vec!['a', 'b'], vec!['c', 'd']];

        let a = Iter::<(), _>::new(a.iter());
        let b = Iter::<_, _>::new(b.iter());
        let c = Iter::<_, _>::new(c.iter());

        a.zip(b).zip(c).for_each_inner(|t| {
            println!("{:?}", t);
        });

        // panic!("test complete");
    }
}
