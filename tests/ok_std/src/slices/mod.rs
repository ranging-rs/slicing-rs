#![cfg(test)]

/// Unpacked bool slice.
mod bool_slice_tests {
    use any_std::slices::bool_slice::assert_equal_items;
    use ranging::slices::BoolSlice;
    use ranging::slices::Slice;

    #[test]
    fn construct_from_existing_data() {
        any_std::slices::bool_slice::construct_from_existing_data();

        let vector = vec![true, false];
        assert_equal_items(&<BoolSlice<2>>::from_vec(vector), &[true, false]);
    }

    #[test]
    fn new_contains_initial_false() {
        any_std::slices::bool_slice::new_contains_initial_false();
    }

    #[test]
    /// Test that `SliceStorage::new_with_vec()` is empty, regardless of const generic param N.
    fn new_with_vec_is_empty() {
        let bool_slice = <BoolSlice<0>>::new_with_vec();
        assert!(bool_slice.shared_slice().is_empty());
        assert_equal_items(&bool_slice, &[]);

        let bool_slice = <BoolSlice<2>>::new_with_vec();
        assert!(bool_slice.shared_slice().is_empty());
        assert_equal_items(&bool_slice, &[]);

        let vec_of_four = Vec::<u32>::with_capacity(4);
        assert!(vec_of_four.capacity()>=4);
        //println!("vec_of_four: {}", vec_of_four.len());
        //assert!(false)
    }

    #[test]
    fn get() {}
}
