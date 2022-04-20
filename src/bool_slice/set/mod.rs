use crate::bool_flag::BoolFlagSet;
use crate::byte_slice::ByteSliceBoolStorage;
use crate::slices::BoolSlice;

/// Backed by a slice of booleans (not packed, but ordinary).
pub type UnpackedSet<'s, T, I, const N: usize> = BoolFlagSet<'s, T, I, BoolSlice<'s, N>, N>;

pub type PackedSet<'s, T, I, const N: usize> =
    BoolFlagSet<'s, T, I, ByteSliceBoolStorage<'s, N>, N>;
