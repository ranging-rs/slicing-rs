#![cfg_attr(feature = "no_std", no_std)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]

pub mod map;
pub mod set;

use core::fmt::{self};
use core::ops::{Add, Sub};

#[derive(Debug)]
pub enum Slice<'a, T: 'a> {
    Shared(&'a [T]),
    Mutable(&'a mut [T]),
    #[cfg(not(no_std))]
    Vec(Vec<T>),
}

impl<'s, T: 's> Slice<'s, T> {
    #[inline]
    pub fn shared_slice<'a>(&'a self) -> &'a [T] {
        match &self {
            Slice::Shared(slice) => slice,
            Slice::Mutable(slice) => slice,
            #[cfg(not(no_std))]
            Slice::Vec(vec) => vec,
        }
    }

    #[inline]
    pub fn mutable_slice<'a>(&'a mut self) -> &'a mut [T] {
        match self {
            Slice::Shared(_) => {
                unreachable!("Can't get a mutable slice from a shared slice.")
            },
            Slice::Mutable(slice) => slice,
            #[cfg(not(no_std))]
            Slice::Vec(vec) => vec,
        }
    }
}

pub type BoolSlice<'a> = Slice<'a, bool>;
pub type ByteSlice<'a> = Slice<'a, u8>;

#[allow(unused)]
#[cfg(test)]
mod test {
    #[test]
    fn f() {
        let shared_num = [1, 2, 3];
        let mut mut_num = [4, 5, 6];
    }
}

/// Abstract set.
pub trait Set<T: core::hash::Hash + Eq + Clone>: Clone {
    // To use with non-cloneable, have:
    // type ITER<'a>: Iterator<Item = &'a T>
    // where
    //     T: 'a,
    //     Self: 'a;
    // -- but then we can't have BoolSlice-based or any other value generation.
    /// Thanks to Shadow0133 for https://www.reddit.com/r/rust/comments/t4egmf/lifetime_generic_associated_type_bounded_by_the
    type ITER<'a>: Iterator<Item = T>
    where
        T: 'a,
        Self: 'a;

    fn contains(&self, value: &T) -> bool;
    fn insert(&mut self, value: T) -> bool;
    fn insert_all(&mut self, iter: impl Iterator<Item = T>) {
        iter.for_each(|item| {
            self.insert(item);
        });
    }
    fn remove(&mut self, value: &T) -> bool;
    fn iter<'a>(&'a self) -> Self::ITER<'a>;
    /// Return a new empty set. For range/max size-bound sets it will have same constraints or capacity.
    fn new_like(&self) -> Self;
}

/// Handles transformations of an item to an index, and vice versa.
/// A collection has an `Indexer` instance, but the `Indexer` implementation
/// doesn't know anything about the size/capacity of the collection. So even
/// though `Indexer`'s `key(...)` function may succeed, a particular collection
/// impplementation, or a particular collection instance, may not be able to accept it.
/// It requires `Clone`, so that collections can store `Indexer` instances themselves
/// rather than references (to avoid CPU cache fragmentation).
pub trait Indexer<T: Clone>: Clone {
    /// 0-based index, but specific to collection(s) indexed by this `Indexer` instance. That may be shifted from `RangeIndexable::index()`.
    fn index(&self, key: &T) -> usize;
    /// Used to generate an item (key) when iterating over a boolean-backed or similar set.
    fn key(&self, index: usize) -> T;
    fn new(start_key: &T) -> Self;
}

#[derive(Clone)]
pub struct RangeIndexer<T: Clone> {
    start_key: T,
    /// "Absolute" index respective to (value stored in_ start_key.
    start_index: usize,
}
/// Default implementation for primitive unsigned/signed integers.
/// In nightly Rust as of early 2022, this works for `char`, too - `char` implements `Sub<char>`, even though that doesn't show up at https://doc.rust-lang.org/nightly/std/primitive.char.html.
/// TODO make this compile conditionally: - errornous for 32 bit and bigger integers on 16bit platforms.
impl<T: Clone + Sub<T> + Add<T>> Indexer<T> for RangeIndexer<T>
where
    T: TryInto<usize>,
    usize: TryFrom<T>,
    usize: TryFrom<<T as Sub>::Output>,
    T: TryFrom<usize>,
    <usize as TryFrom<<T as Sub>::Output>>::Error: fmt::Debug,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    <T as TryInto<usize>>::Error: fmt::Debug,
{
    fn index(&self, key: &T) -> usize {
        // @TODO Consider an alternative: key.clone().try_into().expect(...) - self.start_index. Unsure about default implementation for `char`.
        // However, the current implementation would work on 16 bit platforms,
        // while using key.clone.try_into().expect(...) - self.start_index would not!
        (key.clone() - self.start_key.clone())
            .try_into()
            .expect("Item out of range.")
    }
    fn key(&self, index: usize) -> T {
        (self.start_index + index)
            .try_into()
            .expect("Index out of range.")
    }
    fn new(start_key: &T) -> Self {
        Self {
            start_key: start_key.clone(),
            start_index: start_key
                .clone()
                .try_into()
                .expect("Start index out of range."),
        }
    }
}

/*
/// As per https://doc.rust-lang.org/std/primitive.char.html#method.from_u32, any `char` can be cast to u32
/// TODO make this conditional - not compilable on 16bit platforms.
impl Indexer<char> for RangeIndexer<char> {
    fn index(&self, item: &char) -> usize {
        *item as u32 - self.start as u32
    }
    fn value(&self, index: usize) -> char {
        char::from_u32(self.start as usize + index).unwrap()
    }
}*/

fn test_char_range(indexer: &RangeIndexer<char>) {
    let clone = indexer.clone();
}

/// TODO use?
/// Implement only for types where any value has a valid (and unique) usize index.
/// Why not just handle this in `Indexer` implementations? Because this is useful so that users can implement it for their types, without re-implementing `Indexer`.
pub trait Indexable {
    /// "Absolute" index, unique per value. Independent/not specific to a start key of any collection, neither to its capacity.
    fn index(&self) -> usize;
    fn key(index: usize) -> Self;
}
/// TODO implement Indexable for any data-less enum?

/// @TODO use?
/// Like `Indexable`, this is useful for user-defined types.
pub trait RangeIndexable: Clone {
    fn index(&self, base: &Self) -> usize;
    /// Intentionally not using &self or `base: &Self` parameter instead of `indexer`, since it could be unclear.
    fn key(index: usize, indexer: &RangeIndexer<Self>) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
