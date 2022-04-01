use crate::index::Indexer;
use core::marker::PhantomData;

pub trait Slice<'a, T: 'a + Clone + PartialEq> {
    fn get(&self, index: usize) -> T;
    /// Return true if this value was not present yet. (Based on std::collections::HashSet.)
    fn set(&mut self, index: usize, value: &T) -> bool;
    fn iter(&'a self) -> core::slice::Iter<'a, T>;
}

/// Const generic param `N` is used by `Slice::Array` only. (However, it makes all variants consume space. Hence:) Suggested for `no_std` only.
/// If you run in `std`, suggest passing 0 for `N`, and use `Slice::Vec` instead.
#[derive(Debug)]
pub enum SliceStorage<'a, T: 'a, const N: usize> {
    Shared(&'a [T]),
    Mutable(&'a mut [T]),
    /// Owned array. Suggested for no_std only.
    Array([T; N]),
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    /// Owned vector. For std only.
    Vec(Vec<T>),
}

impl<'s, T: 's, const N: usize> SliceStorage<'s, T, N> {
    #[inline]
    /*pub*/
    fn shared_slice<'a>(&'a self) -> &'a [T] {
        match &self {
            SliceStorage::Shared(slice) => slice,
            SliceStorage::Mutable(slice) => slice,
            SliceStorage::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => vec,
        }
    }

    #[inline]
    /// Implemented for all except for Shared-based slice.
    /*pub*/
    fn mutable_slice<'a>(&'a mut self) -> &'a mut [T] {
        match self {
            SliceStorage::Shared(_) => {
                unimplemented!("Can't get a mutable slice from a shared slice.")
            }
            SliceStorage::Mutable(slice) => slice,
            SliceStorage::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => vec,
        }
    }
}

impl<'s, T: 's + Clone + PartialEq, const N: usize> Slice<'s, T> for SliceStorage<'s, T, N> {
    fn get(&self, index: usize) -> T {
        self.shared_slice()[index].clone()
    }
    fn set(&mut self, index: usize, value: &T) -> bool {
        let mutable_slice = self.mutable_slice();
        let is_modifying = *value != mutable_slice[index];
        mutable_slice[index] = value.clone();
        is_modifying
    }
    fn iter(&'s self) -> core::slice::Iter<'s, T> {
        self.shared_slice().iter()
    }
}

impl<'s, T: 's + Clone, const N: usize> Clone for SliceStorage<'s, T, N> {
    /// Implemented for Array-backed and Vec-backed slice only.
    fn clone(&self) -> Self {
        match self {
            SliceStorage::Shared(_) | SliceStorage::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            SliceStorage::Array(array) => SliceStorage::Array(array.clone()),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => SliceStorage::Vec(vec.clone()),
        }
    }
}

impl<'s, T: 's + Clone + Copy + Default, const N: usize> crate::abstra::NewLike
    for SliceStorage<'s, T, N>
{
    /// Implemented for Array-backed and Vec-backed slice only.
    fn new_like(&self) -> Self {
        match self {
            SliceStorage::Shared(_) | SliceStorage::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            SliceStorage::Array(_) => SliceStorage::Array([T::default(); N]),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(_) => SliceStorage::Vec(if N > 0 {
                Vec::with_capacity(N)
            } else {
                Vec::new()
            }),
        }
    }
}

pub type BoolSlice<'a, const N: usize> = SliceStorage<'a, bool, N>;
pub type ByteSlice<'a, const N: usize> = SliceStorage<'a, u8, N>;
