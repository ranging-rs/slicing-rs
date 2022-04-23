use slicing::byte_slice::ByteSliceBoolStorage;
use slicing::index::{Indexer, RangeIndexer};
use slicing::slices::{BoolSlice, SliceDefault};

fn indexer_usize() -> RangeIndexer<usize> {
    RangeIndexer::new(&0)
}

pub fn unpacked() {
    let indexer = indexer_usize();
}

pub fn packed() {
    let indexer = indexer_usize();
}

#[cfg(all(not(feature = "no_std"), feature = "std"))]
pub fn hashed() {
    let indexer = indexer_usize();
}

#[cfg(test)]
mod tests {}
