use crate::bool_flag::BoolFlagSet;
use crate::slices::BoolSlice;

/// Backed by a slice of booleans (not packed, but ordinary).
pub type Set<'s, T, I, const N: usize> = BoolFlagSet<'s, T, I, BoolSlice<'s, N>, N>;
