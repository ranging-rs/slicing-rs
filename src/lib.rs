#![cfg_attr(feature = "no_std", no_std)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]

pub mod bool_array;
pub mod bool_slice;
mod set;
mod slices;

/// TODO why doesn't the following activate?
//#[cfg(not(no_std))]
//#[cfg(not(feature = "no_std"))]
// something-to-conditionally-compile
pub mod hash;

pub use set::*;
pub use slices::*;

use core::fmt::{self};
use core::ops::{Add, Sub};

#[allow(unused)]
#[cfg(test)]
mod test {
    #[test]
    fn f() {}
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
/// TODO Remove Add<T>
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
