use super::*;
use std::marker::PhantomData;

pub struct Zip<A, T, U> {
    t: T,
    u: U,
    marker: PhantomData<A>,
}

impl<A, T, U> Zip<A, T, U> {
    pub(super) fn new(t: T, u: U) -> Self {
        Self {
            t,
            u,
            marker: PhantomData,
        }
    }
}

impl<A, T, U> IntoIterator for Zip<A, T, U>
where
    T: IterOver<Type = A>,
    U: IterOver<Type = A>,
{
    type Item = (T::Item, U::Item);
    type IntoIter = std::iter::Zip<T::IntoIter, U::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.t.into_iter().zip(self.u.into_iter())
    }
}

impl<A, T: IterOver<Type = A>, U: IterOver<Type = A>> IterOver for Zip<A, T, U> {
    type Type = A;
}
