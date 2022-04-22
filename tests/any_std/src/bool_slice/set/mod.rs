use ranging::bool_slice::set::PackedSet;
use ranging::bool_slice::set::UnpackedSet;
use ranging::byte_slice::ByteSliceBoolStorage;
use ranging::index::{Indexer, RangeIndexer};
use ranging::set::Set;
use ranging::slices::{BoolSlice, SliceDefault};

pub fn common(set: &mut impl Set<usize>) {
    assert!(!set.contains(&2));
    let two_was_not_present_initially = set.insert(2);
    assert!(two_was_not_present_initially);
    assert!(set.contains(&2));

    let two_was_readded = set.insert(2);
    assert!(!two_was_readded);

    let two_was_present_before_remove = set.remove(&2);
    assert!(two_was_present_before_remove);
    //debug_assert!()
}

pub fn unpacked() {
    let indexer = RangeIndexer::<usize>::new(&0);
    let mut set = UnpackedSet::new(BoolSlice::<10>::from_default_to_array(), indexer);

    common(&mut set);
}

pub fn packed() {
    let indexer = RangeIndexer::<usize>::new(&0);
    let mut set = PackedSet::new(ByteSliceBoolStorage::<10>::from_default_to_array(), indexer);

    common(&mut set);
}

#[cfg(test)]
mod tests {}
