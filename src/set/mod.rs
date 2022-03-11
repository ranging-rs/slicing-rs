use super::{BoolSlice, ByteSlice, Indexer, RangeIndexer};
use core::hash::Hash;
use core::ops::{Add, Sub};
use core::{fmt, marker::PhantomData};
#[cfg(not(no_std))]
use std::collections::{hash_set, HashSet};

#[derive(Debug)]
pub struct HashedSet<T> {
    set: HashSet<T>,
}

impl<T: Hash + Eq + Clone> super::Set<T> for HashedSet<T> {
    type ITER<'a>
    where
        T: 'a,
        Self: 'a,
    = HashedSetIter<'a, T>;
    fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
    fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }
    fn remove(&mut self, value: &T) -> bool {
        self.set.remove(value)
    }
    fn iter<'a>(&'a self) -> Self::ITER<'a> {
        HashedSetIter {
            set_iter: self.set.iter(),
        }
    }

    fn new_like(&self) -> Self {
        Self {
            set: HashSet::<T>::new(),
        }
    }
}

impl<T: Hash + Eq + Clone> Clone for HashedSet<T> {
    fn clone(&self) -> Self {
        Self {
            set: self.set.clone(),
        }
    }
}

impl<T: Hash + Eq> HashedSet<T> {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
}

pub struct HashedSetIter<'a, T: 'a> {
    set_iter: hash_set::Iter<'a, T>,
}
impl<'a, T: Clone> Iterator for HashedSetIter<'a, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        self.set_iter.next().map(|value| value.clone())
    }
}

impl<T: core::hash::Hash + Eq> FromIterator<T> for HashedSet<T> {
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = T>,
    {
        Self {
            set: HashSet::from_iter(iter),
        }
    }
}
// --------------------------------

/// Backed by a slice of booleans (not packed, but ordinary).
/// Not backed by an (owned) array - that would require a const generic parameter (size), which would
/// enlarge the resulting binary and compile & build time.
#[derive(Debug)]
pub struct BoolSliceBackedSet<'s, T: Clone, I: Indexer<T>> {
    slice: BoolSlice<'s>,
    /// Stored owned, not by reference - good for CPU cache affinity.
    indexer: I,
    _items: PhantomData<T>, // so that we don't mix BoolSliceSet of various item types
}

impl<'s, T: Eq + Clone, I: Indexer<T>> BoolSliceBackedSet<'s, T, I> {
    #[inline]
    fn shared_slice_OLD<'a>(&'a self) -> &'a [bool] {
        match &self.slice {
            BoolSlice::Shared(slice) => slice,
            BoolSlice::Mutable(slice) => slice,
            BoolSlice::Vec(vec) => vec,
        }
    }
    #[inline]
    fn shared_slice<'a>(&'a self) -> &'a [bool] {
        self.slice.shared_slice()
    }
}

impl<'s, T: Hash + Eq + Clone, I: Indexer<T>> super::Set<T> for BoolSliceBackedSet<'s, T, I> {
    type ITER<'a>
    where
        T: 'a,
        Self: 'a,
    = BoolSliceBackedSetIter<'a, T, I>;
    fn contains(&self, value: &T) -> bool {
        self.shared_slice()[self.indexer.index(value)]
    }
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
    fn remove(&mut self, value: &T) -> bool {
        match &mut self.slice {
            BoolSlice::Mutable(slice) => {
                let index = self.indexer.index(&value);
                let was_present = slice[index];
                slice[index] = false;
                was_present
            }
            _ => unimplemented!("Based on a shared reference - read only."),
        }
    }
    fn iter<'a>(&'a self) -> Self::ITER<'a> {
        BoolSliceBackedSetIter {
            slice_enum: self.shared_slice().iter().enumerate(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
        }
    }
    fn new_like(&self) -> Self {
        unimplemented!("Cannot be implemented.");
    }
}

impl<'s, T: Hash + Eq + Clone, I: Indexer<T>> Clone for BoolSliceBackedSet<'s, T, I> {
    fn clone(&self) -> Self {
        unimplemented!("Cannot be supported");
    }
}

#[derive(Clone)]
pub struct BoolSliceBackedSetIter<'a, T: Clone, I: Indexer<T>> {
    slice_enum: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
    /// Not a reference, but cloned & owned (better for CPU cache affinity)
    /// when indexers are small. Having a big indexer? Then make the Indexer implementation refer to it.
    indexer: I,
    _items: PhantomData<T>,
}
impl<'a, T: Clone, I: Indexer<T>> Iterator for BoolSliceBackedSetIter<'a, T, I> {
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

impl<'s, T: Eq + Clone + Sub<T> + Add<T>> BoolSliceBackedSet<'s, T, RangeIndexer<T>>
where
    T: TryInto<usize>,
    usize: TryFrom<T>,
    usize: TryFrom<<T as Sub>::Output>,
    T: TryFrom<usize>,
    <usize as TryFrom<<T as Sub>::Output>>::Error: fmt::Debug,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    <T as TryInto<usize>>::Error: fmt::Debug,
{
    pub fn new(slice: &'s [bool], start: &T) -> Self {
        Self {
            slice: BoolSlice::Shared(slice),
            indexer: RangeIndexer::<T>::new(start),
            _items: PhantomData,
        }
    }
}
