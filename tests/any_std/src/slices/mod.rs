/// Helpers for (unpacked) bool slice. Used both by tests in this project, and by tests in `ok_std/` and `no_std_*/` projects.
pub mod bool_slice {
    use ranging::slices::BoolSlice;
    use ranging::slices::SliceClone;
    use ranging::slices::SliceStorage;

    /// Assert that `bool_based_slice` has same size and items as `slice`.
    pub fn assert_equal_items<const N: usize>(
        bool_based_slice: &SliceStorage<bool, N>,
        slice: &[bool],
    ) {
        let inner_slice = bool_based_slice.shared_slice();

        assert_eq!(inner_slice.len(), slice.len());
        // Can't use `assert_eq!(inner_slice, slice);` because that uses
        // PartialEq for slices, which use `slice/cmp.rs`, which uses `memcmp`,
        // which doesn't exist in `no_std`.
        for i in 0..slice.len() {
            assert_eq!(inner_slice[i], slice[i]);
        }
    }

    pub fn construct_from_existing_data() {
        let mut array = [true, false];

        assert_equal_items(&(BoolSlice::<2>::from_shared(&array)), &[true, false]);
        assert_equal_items(&(BoolSlice::<2>::from_mutable(&mut array)), &[true, false]);
        assert_equal_items(&BoolSlice::<2>::from_array(array), &[true, false]);
    }

    pub fn new_contains_initial_false() {
        assert_equal_items(&BoolSlice::<1>::new_with_array(), &[false]);
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
