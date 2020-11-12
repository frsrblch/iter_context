use super::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Zip<C, T, U> {
    t: T,
    u: U,
    marker: PhantomData<C>,
}

impl<C, T, U> Zip<C, T, U> {
    pub(super) fn new(t: T, u: U) -> Self {
        Self {
            t,
            u,
            marker: PhantomData,
        }
    }
}

impl<C, T, U> IntoIterator for Zip<C, T, U>
where
    T: ContextualIterator<Context = C>,
    U: ContextualIterator<Context = C>,
{
    type Item = (T::Item, U::Item);
    type IntoIter = std::iter::Zip<T::IntoIter, U::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.t.into_iter().zip(self.u.into_iter())
    }
}

impl<C, T, U> ContextualIterator for Zip<C, T, U>
where
    T: ContextualIterator<Context = C>,
    U: ContextualIterator<Context = C>,
{
    type Context = C;
}
