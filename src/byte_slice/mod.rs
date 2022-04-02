use crate::bool_flag::BoolFlagSet;
use crate::index::Indexer;
use crate::slices::{ByteSlice, Slice};

pub struct ByteSliceBoolStorage<'a, const N: usize>
where
    Self: 'a,
{
    byte_slice: ByteSlice<'a, N>,
}

/// "one shifted": Return 1u8, shifted by `index` places to left.
const fn os(shift: usize) -> u8 {
    1 << shift
}
/// "zero shifted": Return 1u8, shifted by `index` places to left, and negated.
const fn zs(shift: usize) -> u8 {
    !(os(shift))
}
const ONE_SHIFTS: [u8; 8] = [!1, os(1), os(2), os(3), os(4), os(5), os(6), os(7)];
/// negated values of ONE_SHIFTS
const ZERO_SHIFTS: [u8; 8] = [!1, zs(1), zs(2), zs(3), zs(4), zs(5), zs(6), zs(7)];

impl<'a, const N: usize> ByteSliceBoolStorage<'a, N> {
    /// Return (byte_index, old_byte, new_byte)
    fn dry_run_set(&self, index: usize, value: &bool) -> (usize, u8, u8) {
        let byte_index = index / 8;
        let bit_subindex = index % 8;
        let old_byte = self.byte_slice.get(index / 8);

        let new_byte = if *value {
            old_byte | ONE_SHIFTS[bit_subindex]
        } else {
            old_byte & ZERO_SHIFTS[bit_subindex]
        };
        (byte_index, old_byte, new_byte)
    }
}

impl<'a, const N: usize> Slice<'a, bool> for ByteSliceBoolStorage<'a, N> {
    type ITER<'s> = core::slice::Iter<'s, bool>
    where Self: 's;
    fn get(&self, index: usize) -> bool {
        let byte = self.byte_slice.get(index / 8);
        let one_shifted = ONE_SHIFTS[index % 8];
        (byte & one_shifted) != 0
    }
    fn set(&mut self, index: usize, value: &bool) {
        let (byte_index, _, new_byte) = self.dry_run_set(index, value);
        self.byte_slice.set(byte_index, &new_byte);
    }
    fn check_and_set(&mut self, index: usize, value: &bool) -> bool {
        let (byte_index, old_byte, new_byte) = self.dry_run_set(index, value);
        self.byte_slice.set(byte_index, &new_byte);
        old_byte != new_byte
    }
    fn iter<'s>(&'s self) -> Self::ITER<'s> {
        todo!()
    }
}

/// Backed by a packed slice of bits (rounded up to bytes). That results not only in 8x less storage,  but in less cache & RAM bandidth => faster.

pub type Set<'s, T, I, const N: usize> = BoolFlagSet<'s, T, I, ByteSliceBoolStorage<'s, N>, N>;
