#![cfg(test)]

/// Unpacked bool slice.
mod bool_slice_tests {

    #[test]
    fn construct_from_existing_data() {
        any_std::slices::bool_slice::construct_from_existing_data();
    }

    #[test]
    fn new_contains_initial_false() {
        any_std::slices::bool_slice::new_contains_initial_false();
    }
}
