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

fn get_bit(byte: u8, bit_subindex: usize) -> bool {
    let one_shifted = ONE_SHIFTS[bit_subindex];
    (byte & one_shifted) != 0
}

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

impl<'a, const N: usize> Slice<'a, bool, N> for ByteSliceBoolStorage<'a, N> {
    //type ITER<'s> = core::slice::Iter<'s, bool>
    type ITER<'s> = ByteSliceBoolIter<'s>
    where Self: 's;
    fn get(&self, index: usize) -> bool {
        let byte = self.byte_slice.get(index / 8);
        get_bit(byte, index % 8)
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
        ByteSliceBoolIter::new(self.byte_slice.iter())
    }
    // Constructor functions. Supposed to be in-place/copy, but that's not possible from bool-based input - hence never to be implemented.
    fn from_shared_slice(_slice: &'a [bool]) -> Self {
        unimplemented!("Never")
    }
    fn from_mutable_slice(_slice: &'a mut [bool]) -> Self {
        unimplemented!("Never")
    }
    fn from_array(_array: [bool; N]) -> Self {
        unimplemented!("Never")
    }
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec(_vector: Vec<bool>) -> Self {
        unimplemented!("Never")
    }
    fn new_with_array() -> Self {
        unimplemented!("Never")
    }
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn new_with_vec() -> Self {
        unimplemented!("Never")
    }
}

/// Backed by a packed slice of bits (rounded up to bytes). That results not only in 8x less storage,  but in less cache & RAM bandidth => faster.

pub type Set<'s, T, I, const N: usize> = BoolFlagSet<'s, T, I, ByteSliceBoolStorage<'s, N>, N>;

pub struct ByteSliceBoolIter<'a> {
    /// Next index into current_byte. Always valid (<8) if `current_byte` is valid, too. It could be u8, but conversions would make the code cluttered.
    bit_subindex: usize,
    ///
    current_byte: Option<&'a u8>,
    byte_slice_it: core::slice::Iter<'a, u8>,
}

impl<'a> Iterator for ByteSliceBoolIter<'a> {
    type Item = &'a bool;

    #[inline]
    fn next(&mut self) -> Option<&'a bool> {
        assert!(self.bit_subindex < 8);

        match self.current_byte {
            Some(&current_byte) => {
                let result = get_bit(current_byte, self.bit_subindex);
                if self.bit_subindex < 7 {
                    self.bit_subindex += 1;
                } else {
                    self.bit_subindex = 0;
                    // A little eager, but that's OK with a slice-backed iter.
                    self.current_byte = self.byte_slice_it.next();
                }
                Some(if result { &true } else { &false })
            }
            None => {
                // At the very end.
                None
            }
        }
    }
}

impl<'a> ByteSliceBoolIter<'a> {
    fn new(mut slice_it: core::slice::Iter<'a, u8>) -> Self {
        Self {
            bit_subindex: 0,
            current_byte: slice_it.next(),
            byte_slice_it: slice_it,
        }
    }
}
