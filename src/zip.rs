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
    T: TypedIterator<Context = A>,
    U: TypedIterator<Context = A>,
{
    type Item = (T::Item, U::Item);
    type IntoIter = std::iter::Zip<T::IntoIter, U::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.t.into_iter().zip(self.u.into_iter())
    }
}

impl<A, T: TypedIterator<Context = A>, U: TypedIterator<Context = A>> TypedIterator
    for Zip<A, T, U>
{
    type Context = A;
}
