//! Iterators for use with collections like [Map], [Vec].
#[cfg(doc)]
use crate::{Map, Vec};

use core::fmt::Debug;
use core::iter::FusedIterator;
use core::marker::PhantomData;

pub trait UnwrappedEnumerable<I, T, E> {
    fn unwrapped(self) -> UnwrappedIter<I, T, E>;
}

impl<I, T, E> UnwrappedEnumerable<I, T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
    E: Debug,
{
    fn unwrapped(self) -> UnwrappedIter<I, T, E> {
        UnwrappedIter {
            iter: self,
            item_type: PhantomData,
            error_type: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct UnwrappedIter<I, T, E> {
    iter: I,
    item_type: PhantomData<T>,
    error_type: PhantomData<E>,
}

impl<I, T, E> Iterator for UnwrappedIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
    E: Debug,
{
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Result::unwrap)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, T, E> DoubleEndedIterator for UnwrappedIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>> + DoubleEndedIterator,
    E: Debug,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Result::unwrap)
    }
}

impl<I, T, E> FusedIterator for UnwrappedIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>> + FusedIterator,
    E: Debug,
{
}

impl<I, T, E> ExactSizeIterator for UnwrappedIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>> + ExactSizeIterator,
    E: Debug,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}
