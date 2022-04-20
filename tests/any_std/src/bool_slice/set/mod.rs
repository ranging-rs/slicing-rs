use ranging::set::Set;
use ranging::bool_slice::set::PackedSet;
use ranging::bool_slice::set::UnpackedSet;
use ranging::index::{Indexer, RangeIndexer};
ranging::byte_slice::ByteSliceBoolStorage;

pub fn unpacked() {
    let indexer = RangeIndexer::<usize>::new(&0);
    let mut set = UnpackedSet::new(ByteSliceBoolStorage);
}

#[cfg(test)]
mod tests {

}