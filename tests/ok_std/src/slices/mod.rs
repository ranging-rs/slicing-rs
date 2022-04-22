#![cfg(test)]

/// Unpacked bool slice.
mod bool_slice_tests {
    use any_std::slices::bool_slice::assert_equal_items;
    use ranging::slices::BoolSlice;
    use ranging::slices::{Slice, SliceDefault};

    #[test]
    fn construct_from_existing_data() {
        any_std::slices::bool_slice::construct_from_existing_data();

        // std-specific
        // @TODO move to any_std/ and cfg conditionally compile
        let vector = vec![true, false];
        assert_equal_items(&<BoolSlice<2>>::from_vec(vector), &[true, false]);
    }

    #[test]
    fn new_contains_initial_false() {
        any_std::slices::bool_slice::new_contains_initial_false();
    }

    #[test]
    fn from_vec_etc() {
        // Test that `SliceStorage::from_vec_new()` is empty, regardless of const generic param N.
        let mut bool_slice = BoolSlice::<0>::from_vec_new();
        assert!(bool_slice.shared_slice().is_empty());
        assert_eq!(bool_slice.mutable_vec().capacity(), 0);
        assert_equal_items(&bool_slice, &[]);

        let mut bool_slice = <BoolSlice<2>>::from_vec_with_capacity(2);
        assert!(bool_slice.shared_slice().is_empty());
        assert!(bool_slice.mutable_vec().capacity() >= 2);
        assert_equal_items(&bool_slice, &[]);

    }

    #[test]
    fn get() {}
}
