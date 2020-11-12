use crate::ContextualIterator;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Map<C, I, F> {
    iter: I,
    f: F,
    marker: PhantomData<C>,
}

impl<C, I, F> Map<C, I, F> {
    pub(super) fn new(iter: I, f: F) -> Self {
        Map {
            iter,
            f,
            marker: PhantomData,
        }
    }
}

impl<C, I, F, U> IntoIterator for Map<C, I, F>
where
    I: IntoIterator,
    F: FnMut(<I as IntoIterator>::Item) -> U,
{
    type Item = U;
    type IntoIter = std::iter::Map<I::IntoIter, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().map(self.f)
    }
}

impl<C, I, F> ContextualIterator for Map<C, I, F>
where
    Self: IntoIterator,
{
    type Context = C;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map() {
        let a = [1u32, 2, 3];
        let iter = crate::Iter::<(), u32>::new(a.iter());
        let map = iter.map(|i| i * 2);

        let b = [2u32, 4, 6];
        let iter = crate::Iter::<(), u32>::new(b.iter());

        map.zip(iter).for_each(|(a, &b)| {
            assert_eq!(a, b);
        });
    }
}
