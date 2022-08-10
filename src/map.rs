use crate::{ContextualIterator, Zip};
use std::marker::PhantomData;

/// Maps the values from a [ContextualIterator] with the given closure, which is itself
/// a [ContextualIterator] with the same Context as the original iterator.
#[derive(Clone)]
pub struct Map<Context, IntoIter, F> {
    into_iter: IntoIter,
    f: F,
    context: PhantomData<Context>,
}

impl<Context, IntoIter, F> Map<Context, IntoIter, F> {
    pub(super) fn new(iter: IntoIter, f: F) -> Self {
        Map {
            into_iter: iter,
            f,
            context: PhantomData,
        }
    }
}

impl<Context, IntoIter, F, IterItem, Output> IntoIterator for Map<Context, IntoIter, F>
where
    IntoIter: IntoIterator<Item = IterItem>,
    F: FnMut(IterItem) -> Output,
{
    type Item = Output;
    type IntoIter = std::iter::Map<IntoIter::IntoIter, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter.into_iter().map(self.f)
    }
}

impl<Context, IntoIter, F> ContextualIterator for Map<Context, IntoIter, F>
where
    Self: IntoIterator,
{
    type Context = Context;
}

macro_rules! impl_op {
    ($op_trait:ident, $op_fn:ident) => {
        impl<Context, IntoIter, F, Item, Output, Rhs, RhsItem> std::ops::$op_trait<Rhs>
            for Map<Context, IntoIter, F>
        where
            Self: IntoIterator<Item = Item>,
            Rhs: ContextualIterator<Context = Context, Item = RhsItem>,
            Item: std::ops::$op_trait<RhsItem, Output = Output>,
        {
            type Output = Map<Context, Zip<Context, Self, Rhs>, fn((Item, RhsItem)) -> Output>;

            fn $op_fn(self, rhs: Rhs) -> Self::Output {
                self.zip(rhs).map(|(lhs, rhs)| lhs.$op_fn(rhs))
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op!(Rem, rem);
impl_op!(Shl, shl);
impl_op!(Shr, shr);
impl_op!(BitAnd, bitand);
impl_op!(BitOr, bitor);
impl_op!(BitXor, bitxor);

impl<Context, IntoIter, F, Item, Output> std::ops::Neg for Map<Context, IntoIter, F>
where
    Self: IntoIterator<Item = Item>,
    Item: std::ops::Neg<Output = Output>,
{
    type Output = Map<Context, Self, fn(Item) -> Output>;

    fn neg(self) -> Self::Output {
        self.map(std::ops::Neg::neg)
    }
}

impl<Context, IntoIter, F, Item, Output> std::ops::Not for Map<Context, IntoIter, F>
where
    Self: IntoIterator<Item = Item>,
    Item: std::ops::Not<Output = Output>,
{
    type Output = Map<Context, Self, fn(Item) -> Output>;

    fn not(self) -> Self::Output {
        self.map(std::ops::Not::not)
    }
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
