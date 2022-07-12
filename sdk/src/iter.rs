use core::fmt::Debug;
use core::iter::FusedIterator;
use core::marker::PhantomData;

use crate::env::IntoTryFromVal;

pub trait UncheckedEnumerable<I, T> {
    fn unchecked(self) -> UncheckedIter<I, T>;
}

impl<I, T> UncheckedEnumerable<I, T> for I
where
    I: Iterator<Item = Result<T, T::Error>>,
    T: IntoTryFromVal,
    T::Error: Debug,
{
    fn unchecked(self) -> UncheckedIter<I, T> {
        UncheckedIter(self, PhantomData)
    }
}

#[derive(Clone)]
pub struct UncheckedIter<I, T>(I, PhantomData<T>);

impl<I, T> Iterator for UncheckedIter<I, T>
where
    I: Iterator<Item = Result<T, T::Error>>,
    T: IntoTryFromVal,
    T::Error: Debug,
{
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Result::unwrap)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<I, T> DoubleEndedIterator for UncheckedIter<I, T>
where
    I: Iterator<Item = Result<T, T::Error>> + DoubleEndedIterator,
    T: IntoTryFromVal,
    T::Error: Debug,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(Result::unwrap)
    }
}

impl<I, T> FusedIterator for UncheckedIter<I, T>
where
    I: Iterator<Item = Result<T, T::Error>> + FusedIterator,
    T: IntoTryFromVal,
    T::Error: Debug,
{
}

impl<I, T> ExactSizeIterator for UncheckedIter<I, T>
where
    I: Iterator<Item = Result<T, T::Error>> + ExactSizeIterator,
    T: IntoTryFromVal,
    T::Error: Debug,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}
