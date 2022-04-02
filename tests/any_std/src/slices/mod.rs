#![cfg(test)]

/// Unpacked bool slice.
mod bool_slice_tests {
    use crate::slices::BoolSlice;
    use crate::slices::Slice;
    use crate::slices::SliceStorage;

    #[test]
    /// Test that `SliceStorage::new_with_vec()` is empty, regardless of const generic param N.
    fn assert_new_with_vec_is_empty() {
        //let bool_slice = <BoolSlice<2>>::new_with_vec();
        let bool_slice = <SliceStorage<bool, 2> as Slice<bool, 2>>::new_with_vec();
        //assert!(bool_slice.)
    }

    /// Assert that `bool_slice` has size 2 and contains `true`, `false` in that order.
    fn assert_true_false(bool_slice: &BoolSlice<2>) {}

    #[test]
    fn get() {}
}
