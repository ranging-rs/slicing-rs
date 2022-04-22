/// Abstract set, either general or for a given range.
/// While some implementations (like `hash::Set`) could be possible without T: Clone, range-based implementations need `T: Clone` so that they can store range a start/base key itself (rather than a reference). But we also need T: Clone for `hash::Set` to be Clone.
pub trait Set<T: Clone + PartialEq>: Clone + crate::abstra::NewLike {
    /// To use with non-cloneable, we could have a standard iterator over ref:
    /// type ITER<'a>: Iterator<Item = &'a T>
    /// where
    ///     T: 'a,
    ///     Self: 'a;
    /// -- but then we couldn't have BoolSlice-based (or any other) value generation!
    /// Hence an implementation is required to support `ITER`. But supporting
    /// `ITERREF` is optional.
    // Thanks to Shadow0133 for https://www.reddit.com/r/rust/comments/t4egmf/lifetime_generic_associated_type_bounded_by_the
    type ITER<'a>: Iterator<Item = T>
    where
        T: 'a,
        Self: 'a;

    type ITERREF<'a>: Iterator<Item = &'a T>
    where
        T: 'a,
        Self: 'a;

    fn contains(&self, value: &T) -> bool;

    /// Return true if this value was not present yet. (Based on std::collections::HashSet.)
    fn insert(&mut self, value: T) -> bool;

    fn insert_all(&mut self, iter: impl Iterator<Item = T>) {
        iter.for_each(|item| {
            self.insert(item);
        });
    }

    /// Return whether it was actually present. (Based on std::collections::HashSet.)
    fn remove(&mut self, value: &T) -> bool;

    /// Required.
    fn iter<'a>(&'a self) -> Self::ITER<'a>;

    /// Optional. Return `true` if `iter_ref` is supported.
    fn supports_iter_ref() -> bool {
        false
    }

    /// Optional. If supported, then `supports_iter_ref` returns `true`.
    fn iter_ref<'a>(&'a self) -> Self::ITERREF<'a> {
        unimplemented!("Not supported.");
    }
}
