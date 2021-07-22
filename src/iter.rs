use super::*;
use std::marker::PhantomData;

pub struct Iter<'a, C, T> {
    iter: std::slice::Iter<'a, T>,
    marker: PhantomData<C>,
}

impl<'a, C, T> Iter<'a, C, T> {
    pub fn new<IntoIter>(into_iter: IntoIter) -> Self
    where
        IntoIter: IntoIterator<IntoIter = std::slice::Iter<'a, T>>,
    {
        Self {
            iter: into_iter.into_iter(),
            marker: PhantomData,
        }
    }
}

impl<'a, C, T> IntoIterator for Iter<'a, C, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}

impl<C, T> ContextualIterator for Iter<'_, C, T> {
    type Context = C;
}

pub struct IterMut<'a, C, T> {
    iter_mut: std::slice::IterMut<'a, T>,
    marker: PhantomData<C>,
}

impl<'a, C, T> IterMut<'a, C, T> {
    pub fn new<IntoIter>(into_iter: IntoIter) -> Self
    where
        IntoIter: IntoIterator<IntoIter = std::slice::IterMut<'a, T>>,
    {
        Self {
            iter_mut: into_iter.into_iter(),
            marker: PhantomData,
        }
    }
}

impl<'a, C, T> IntoIterator for IterMut<'a, C, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut
    }
}

impl<C, T> ContextualIterator for IterMut<'_, C, T> {
    type Context = C;
}
