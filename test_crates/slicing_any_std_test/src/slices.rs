/// Helpers for (unpacked) bool slice. Used both by tests in this project, and by tests in `ok_std/` and `no_std_*/` projects.
pub mod bool_slice {
    use slicing::slices::BoolSlice;
    use slicing::slices::SliceDefault;
    use slicing::slices::SliceStorageDefault;
    // If this module were in the file of its parent module, do NOT have the
    // following three at the parent module's level. Otherwise macro vec![]
    // would not resolve! (Even more confusing: If you did have `use
    // alloc::vec::Vec` at the parent module level in the same file, struct
    // `Vec` would resolve here (at the child module level) - unlike `vec![]`,
    // which would not resolve here.)
    #[cfg(feature = "no_std_vec")]
    extern crate alloc;
    #[cfg(feature = "no_std_vec")]
    use alloc::vec;

    /// Assert that `bool_based_slice` has same size and items as `slice`.
    fn assert_equal_items<const N: usize>(
        bool_based_slice: &SliceStorageDefault<bool, N>,
        slice: &[bool],
    ) {
        let inner_slice = bool_based_slice.shared_slice();

        assert_eq!(inner_slice.len(), slice.len());
        assert_eq!(inner_slice, slice);
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

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        {
            let vector = vec![true, false];
            assert_equal_items(&<BoolSlice<2>>::from_vec(vector), &[true, false]);
        }
    }

    pub fn new_contains_initial_false() {
        assert_equal_items(&BoolSlice::<1>::from_default_to_array(), &[false]);
    }

    #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
    pub fn from_vec_etc() {
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

    #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
    #[test]
    pub fn from_vec_etc() {
        super::bool_slice::from_vec_etc();
    }
}
