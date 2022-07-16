use crate::with_std;

with_std! {
    use std::{hash::Hash, collections::HashSet};
}

pub trait NewEmptyLike {
    /// Return a new & empty container. For range/max size-bound sets it will
    /// have same constraints or capacity. For some containers it's implemented
    /// only if their item type implements Default. For HashSet/HashMap-based
    /// containers it's implemented regardless of the entry/key type - because
    /// those can be empty.
    fn new_empty_like(&self) -> Self;
}

with_std! {
    impl<T: Hash + Eq + Clone> NewEmptyLike for HashSet<T> {
        fn new_empty_like(&self) -> Self {
            Self::new()
        }
    }
}
