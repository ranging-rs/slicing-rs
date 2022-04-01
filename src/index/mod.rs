use core::fmt::{self};
use core::ops::Sub;

/// Handles transformations of an item to an index, and vice versa.
/// A collection has an `Indexer` instance, but the `Indexer` implementation
/// doesn't know anything about the size/capacity of the collection. So even
/// though `Indexer`'s `key(...)` function may succeed, a particular collection
/// implementation, or a particular collection instance, may not be able to accept it.
/// It requires `Clone`, so that collections can store `Indexer` instances themselves
/// rather than references (to avoid CPU cache fragmentation & ownership handling).
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
impl<T: Clone + Sub<T>> Indexer<T> for RangeIndexer<T>
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
        // An alternative: key.clone().try_into().expect(...) - self.start_index. @TODO Unsure about default implementation for `char`.
        // However, the current implementation would work on 16 bit platforms, too,
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
impl Indexer<char> for RangeIndexer<char> {
    fn index(&self, item: &char) -> usize {
        (*item as u32 - self.start as u32) as usize
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
