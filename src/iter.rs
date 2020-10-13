use super::*;

pub struct Iter<'a, A, T> {
    iter: std::slice::Iter<'a, T>,
    marker: PhantomData<A>,
}

impl<'a, A, T> Iter<'a, A, T> {
    pub fn new(iter: std::slice::Iter<'a, T>) -> Self {
        Self {
            iter,
            marker: PhantomData,
        }
    }
}

impl<'a, A, T> IntoIterator for Iter<'a, A, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}

impl<A, T> IterOver for Iter<'_, A, T> {
    type Type = A;
}

pub struct IterMut<'a, A, T> {
    iter_mut: std::slice::IterMut<'a, T>,
    marker: PhantomData<A>,
}

impl<'a, A, T> IterMut<'a, A, T> {
    pub fn new(iter_mut: std::slice::IterMut<'a, T>) -> Self {
        Self {
            iter_mut,
            marker: PhantomData,
        }
    }
}

impl<'a, A, T> IntoIterator for IterMut<'a, A, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut
    }
}

impl<A, T> IterOver for IterMut<'_, A, T> {
    type Type = A;
}
