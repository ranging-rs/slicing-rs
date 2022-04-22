use ranging::bool_slice::set::PackedSet;
use ranging::bool_slice::set::UnpackedSet;
use ranging::byte_slice::ByteSliceBoolStorage;
use ranging::index::{Indexer, RangeIndexer};
use ranging::set::Set;
use ranging::slices::{BoolSlice, SliceDefault};

pub fn unpacked() {
    let indexer = RangeIndexer::<usize>::new(&0);
    let mut set = UnpackedSet::new(BoolSlice::<10>::from_default_to_array(), indexer);
    set.contains(&1);

    let two_was_no_present_initially = set.insert(2);
    assert!(two_was_no_present_initially);
    let two_was_present_before_remove = set.remove(&2);
    assert!(two_was_present_before_remove);
}

pub fn packed() {
    let indexer = RangeIndexer::<usize>::new(&0);
    let mut set = PackedSet::new(ByteSliceBoolStorage::<10>::from_default_to_array(), indexer);
}

#[cfg(test)]
mod tests {}
