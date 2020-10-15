pub use crate::iter::{Iter, IterMut};
pub use crate::zip::Zip;

mod iter;
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
