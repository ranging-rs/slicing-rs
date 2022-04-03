/// Helpers for (unpacked) bool slice.
pub mod bool_slice {
    use ranging::slices::BoolSlice;
    use ranging::slices::Slice;

    /// Assert that `bool_slice` has size 2 and contains `true`, `false` in that order.
    /// Param `slice` is not an array (of type `[bool; N]`), but a slice, for flexibility.
    pub fn assert_equal_items<const N: usize>(bool_slice: &BoolSlice<N>, slice: &[bool]) {
        let inner_slice = bool_slice.shared_slice();
        assert_eq!(inner_slice.len(), slice.len());
        assert_eq!(inner_slice, inner_slice);
    }

    pub fn construct_from_existing_data() {
        let mut array = [true, false];
        assert_equal_items(&<BoolSlice<2>>::from_shared_slice(&array), &[true, false]);
        assert_equal_items(
            &<BoolSlice<2>>::from_mutable_slice(&mut array),
            &[true, false],
        );
        assert_equal_items(&<BoolSlice<2>>::from_array(array), &[true, false]);
    }

    pub fn new_contains_initial_false() {
        assert_equal_items(&<BoolSlice<1>>::new_with_array(), &[false]);
    }
}

#[cfg(test)]
mod bool_slice_tests {
    #[test]
    fn construct_from_existing_data() {
        super::bool_slice::construct_from_existing_data();
    }

    #[test]
    fn new_contains_initial_false() {
        super::bool_slice::new_contains_initial_false();
    }
}
