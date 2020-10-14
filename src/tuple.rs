#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Tuple<A, B>(pub A, pub B);

impl<A: IntoIterator, B: IntoIterator> IntoIterator for Tuple<A, B> {
    type Item = Tuple<A::Item, B::Item>;
    type IntoIter = IntoIter<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter(), self.1.into_iter())
    }
}

#[derive(Debug)]
pub struct IntoIter<A, B>(A, B);

impl<A: Iterator, B: Iterator> Iterator for IntoIter<A, B> {
    type Item = Tuple<A::Item, B::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.1.next()?;
        Some(Tuple(a, b))
    }
}
