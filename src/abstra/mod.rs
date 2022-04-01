pub trait NewLike {
    /// Return a new & empty container. For range/max size-bound sets it will have same constraints or capacity.
    /// For some containers it's implemented only if their item type implements Default.
    fn new_like(&self) -> Self;
}
