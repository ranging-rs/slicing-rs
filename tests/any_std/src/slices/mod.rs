/// Helpers for (unpacked) bool slice. Used both by tests in this project, and by tests in `no_std/` project.
pub mod bool_slice {
    use ranging::slices::BoolSlice;
    use ranging::slices::Slice;
    use ranging::slices::SliceStorage;

    /// Assert that `bool_based_slice` has same size and items as `slice`.
    pub fn assert_equal_items<const N: Option<usize>>(
        bool_based_slice: &SliceStorage<bool, N>,
        slice: &[bool],
    ) where
        [(); N.unwrap_or(0)]:,
    {
        let inner_slice = bool_based_slice.shared_slice();
        assert_eq!(inner_slice.len(), slice.len());
        assert_eq!(inner_slice, inner_slice);
    }

    pub fn construct_from_existing_data() {
        let mut array = [true, false];

        assert_equal_items(
            &(BoolSlice::<{ Some(2) }>::from_shared_slice(&array)),
            &[true, false],
        );
        assert_equal_items(
            &(BoolSlice::<{ Some(2) }>::from_mutable_slice(&mut array)),
            &[true, false],
        );
        assert_equal_items(&BoolSlice::<{ Some(2) }>::from_array(array), &[true, false]);
    }

    pub fn new_contains_initial_false() {
        assert_equal_items(&BoolSlice::<{ Some(1) }>::new_with_array(), &[false]);
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
