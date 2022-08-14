#![cfg(test)]

/// Unpacked bool slice.
mod bool_slice_tests {

    #[test]
    fn construct_from_existing_data() {
        slicing_any_std_test::slices::bool_slice::construct_from_existing_data();
    }

    #[test]
    fn new_contains_initial_false() {
        slicing_any_std_test::slices::bool_slice::new_contains_initial_false();
    }

    #[test]
    fn from_vec_etc() {
        slicing_any_std_test::slices::bool_slice::from_vec_etc();
    }

    #[test]
    fn get() {}
}
