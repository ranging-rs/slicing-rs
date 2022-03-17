/// Abstract set, either general or for a given range.
/// While some implementations (like `hash::Set`) could be possible without T: Clone, range-based implementations need it to store range a start/base key itself (rather than a reference). But we also need T: Clone for `hash::Set` to be Clone.
pub trait Set<T: /*core::hash::Hash + Eq +*/ Clone>: Clone {
    // To use with non-cloneable, have:
    // type ITER<'a>: Iterator<Item = &'a T>
    // where
    //     T: 'a,
    //     Self: 'a;
    // -- but then we can't have BoolSlice-based or any other value generation.
    /// Thanks to Shadow0133 for https://www.reddit.com/r/rust/comments/t4egmf/lifetime_generic_associated_type_bounded_by_the
    type ITER<'a>: Iterator<Item = T>
    where
        T: 'a,
        Self: 'a;

    fn contains(&self, value: &T) -> bool;
    fn insert(&mut self, value: T) -> bool;
    fn insert_all(&mut self, iter: impl Iterator<Item = T>) {
        iter.for_each(|item| {
            self.insert(item);
        });
    }
    fn remove(&mut self, value: &T) -> bool;
    fn iter<'a>(&'a self) -> Self::ITER<'a>;
    /// Return a new empty set. For range/max size-bound sets it will have same constraints or capacity.
    fn new_like(&self) -> Self;
}
