use crate::index::{Indexer, RangeIndexer};
use crate::{BoolSlice, ByteSlice};
use core::ops::Sub;
use core::{fmt, marker::PhantomData};

/// Backed by a slice of booleans (not packed, but ordinary).
/// See `BoolSlice` for const generic parameter `N`.
#[derive(Debug)]
pub struct Set<'s, T: Clone, I: Indexer<T>, const N: usize> {
    slice: BoolSlice<'s, N>,
    /// Stored owned, not by reference - good for CPU cache affinity.
    indexer: I,
    _items: PhantomData<T>, // so that we don't mix BoolSliceSet of various item types
}

impl<'s, T: Eq + Clone, I: Indexer<T>, const N: usize> Set<'s, T, I, N> {
    // A private helper.
    #[inline]
    fn shared_slice<'a>(&'a self) -> &'a [bool] {
        //self.slice.shared_slice()
        todo!()
    }
}

impl<'s, T: Eq + Clone + Copy + Default, I: Indexer<T>, const N: usize> crate::Set<T>
    for Set<'s, T, I, N>
{
    type ITER<'a>    = SetIter<'a, T, I> where T: 'a, Self: 'a;

    fn contains(&self, value: &T) -> bool {
        // @TODO partially move to Slice::get()
        self.shared_slice()[self.indexer.index(value)]
    }
    /// Return true if this value was not present yet. (Based on std::collections::HashSet.)
    fn insert(&mut self, value: T) -> bool {
        match &mut self.slice {
            BoolSlice::Mutable(slice) => {
                let index = self.indexer.index(&value);
                let already_present = slice[index];
                slice[index] = true;
                !already_present
            }
            _ => unimplemented!("Based on a shared reference - read only."),
        }
    }
    /// Return whether it was actually present.
    fn remove(&mut self, value: &T) -> bool {
        match &mut self.slice {
            BoolSlice::Mutable(slice) => {
                let index = self.indexer.index(&value);
                let was_present = slice[index];
                slice[index] = false;
                was_present
            }
            // @TODO partially move to Slice::set()
            _ => unimplemented!("Based on a shared reference - read only."),
        }
    }
    // @TODO partially move to Slice::iter()
    fn iter<'a>(&'a self) -> Self::ITER<'a> {
        SetIter {
            slice_enum: self.shared_slice().iter().enumerate(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
        }
    }
}

impl<'s, T: Eq + Clone + Copy + Default, I: Indexer<T>, const N: usize> crate::abstra::NewLike
    for Set<'s, T, I, N>
{
    fn new_like(&self) -> Self {
        Self {
            slice: self.slice.new_like(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
        }
    }
}

impl<'s, T: Eq + Clone, I: Indexer<T>, const N: usize> Clone for Set<'s, T, I, N> {
    fn clone(&self) -> Self {
        Self {
            slice: self.slice.clone(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
        }
    }
}

// @TODO Remove if possible.
#[derive(Clone)]
pub struct SetIter<'a, T: Clone, I: Indexer<T>> {
    slice_enum: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
    /// Not a reference, but cloned & owned (better for CPU cache affinity)
    /// when indexers are small. Having a big indexer? Then make the Indexer implementation refer to it.
    indexer: I,
    _items: PhantomData<T>,
}
impl<'a, T: Clone, I: Indexer<T>> Iterator for SetIter<'a, T, I> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        loop {
            if let Some((index, &value_present)) = self.slice_enum.next() {
                if value_present {
                    break Some(self.indexer.key(index));
                }
            } else {
                break None;
            }
        }
    }
}

impl<'s, T: Eq + Clone + Default + Sub<T>, const N: usize> Set<'s, T, RangeIndexer<T>, N>
where
    T: TryInto<usize>,
    usize: TryFrom<T>,
    usize: TryFrom<<T as Sub>::Output>,
    T: TryFrom<usize>,
    <usize as TryFrom<<T as Sub>::Output>>::Error: fmt::Debug,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    <T as TryInto<usize>>::Error: fmt::Debug,
{
    // internal helper
    fn from_bool_slice(slice: BoolSlice<'s, N>, start: &T) -> Self {
        Self {
            slice,
            indexer: RangeIndexer::<T>::new(start),
            _items: PhantomData,
        }
    }
    pub fn from_shared_slice(slice: &'s [bool], start: &T) -> Self {
        Self::from_bool_slice(BoolSlice::Shared(slice), start)
    }
    pub fn from_mutable_slice(slice: &'s mut [bool], start: &T) -> Self {
        Self::from_bool_slice(BoolSlice::Mutable(slice), start)
    }
    pub fn from_array(array: [bool; N], start: &T) -> Self {
        Self::from_bool_slice(BoolSlice::Array(array), start)
    }
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    pub fn from_vec(vector: Vec<bool>, start: &T) -> Self {
        Self::from_bool_slice(BoolSlice::Vec(vector), start)
    }
    pub fn new_with_array(start: &T) -> Self {
        Self::from_bool_slice(BoolSlice::Array([false; N]), start)
    }
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    pub fn new_with_vec(start: &T) -> Self {
        Self::from_bool_slice(
            BoolSlice::Vec(if N > 0 {
                Vec::with_capacity(N)
            } else {
                Vec::new()
            }),
            start,
        )
    }
}
