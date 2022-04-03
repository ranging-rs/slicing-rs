/// Slice/array/vector-based container.
/// If array-based, the size is fixed at compile time through a const generic param `N`.
/// If slice-based or vec-based, its size can be any, as given at runtime. But for shared/mutable slice-based instances the size is fixed at instantiation.
/// Vec-based instances can be resized.
///
/// Param `N` = array size (if Some). Some(0) is also allowed (for empty arrays).
/// Generally, use `N=None` for most `std` purposes, and for instantiation based on a given slice/vector. Use `N=Some(usize)` for owned array-based instantiation (often on stack, or in `non_std`).
pub trait Slice<'a, T: 'a + Clone + PartialEq, const N: Option<usize>>
where
    Self: 'a,
{
    type ITER<'i>: Iterator<Item = &'i T> = core::slice::Iter<'i, T> where T: 'i, Self: 'i;

    fn get(&self, index: usize) -> T;
    /// Set the value. Return true if this value was not present. (Based on std::collections::HashSet.)
    fn check_and_set(&mut self, index: usize, value: &T) -> bool;
    /// Set the value.
    fn set(&mut self, index: usize, value: &T);
    fn iter<'s>(&'s self) -> Self::ITER<'s>;

    // Ownership transfer constructors.
    fn from_shared_slice(slice: &'a [T]) -> Self;
    fn from_mutable_slice(slice: &'a mut [T]) -> Self;
    fn from_array(array: [T; N.unwrap_or(0)]) -> Self;

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec(vector: Vec<T>) -> Self;

    /// Non-transfer constructor referring to a given `vector`.
    /// The only benefit of this function, as compared to `from_mutable_slice`,
    /// is that we can call `mutable_vec` on this instance.
    /// This function doesn't need a shared/immutable alternative - for that
    /// use simple `from_shared_slice`.
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn for_vec(vector: &'a mut Vec<T>) -> Self;

    // Constructors setting blank/default vaLues.
    /// Implemented only if T: Copy + Default.
    fn new_with_array() -> Self;

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    /// Implemented only if T: Default.
    fn new_with_vec(size: usize) -> Self;

    // Reference/link-based constructors. Ever needed? Couldn't we just pass a shared/mutable reference to the existing Slice instance?
    /*fn to_shared_based<'s>(&'s self) -> Self
    where
        Self: 's + Sized;
    fn to_mutable_based<'s>(&'s mut self) -> Self
    where
        Self: 's + Sized;*/

    // Copy constructors.
    // @TODO Would `copy_to_array_based` be a better name?
    /// Copy to a new array and create an instance with it.
    fn to_array_based(&self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    /// Copy to a new vec and create an instance with it.
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn to_vec_based(&self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    // Again, any need for the following? Couldn't we just pass a &mut to the existing (vec-based) Slice instance?
    // fn to_vec_ref_based(&mut self) -> Self

    // Accessors
    fn shared_slice<'s>(&'s self) -> &'s [T];
    /// Implemented for all except for Shared-based slice.
    fn mutable_slice<'s>(&'s mut self) -> &'s mut [T];
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn mutable_vec<'s>(&'s mut self) -> &'s mut Vec<T>;
}

/// Const generic param `N` is used by `Slice::Array` only. (However, it makes all variants consume space. Hence:) Suggested for `no_std` only.
/// If you run in `std`, suggest passing 0 for `N`, and use `Slice::Vec` instead.
#[derive(Debug)]
pub enum SliceStorage<'a, T: 'a, const N: Option<usize>>
where
    [(); N.unwrap_or(0)]:,
{
    Shared(&'a [T]),
    Mutable(&'a mut [T]),
    /// Owned array. Suggested for stack & `no_std`.
    Array([T; N.unwrap_or(0)]),
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    /// Owned vector. For `std` only.
    Vec(Vec<T>),
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    VecRef(&'a mut Vec<T>),
}

// TODO If we ever need this for non-Copy, then split this, and for non-Copy make `new_with_array()` panic.
impl<'a, T: 'a + Copy + PartialEq + Default, const N: Option<usize>> Slice<'a, T, N>
    for SliceStorage<'a, T, N>
where
    [(); N.unwrap_or(0)]:,
{
    type ITER<'i> = core::slice::Iter<'i, T>
    where T: 'i, Self: 'i;
    fn get(&self, index: usize) -> T {
        self.shared_slice()[index].clone()
    }
    fn check_and_set(&mut self, index: usize, value: &T) -> bool {
        let mutable_slice = self.mutable_slice();
        let is_modifying = *value != mutable_slice[index];
        mutable_slice[index] = value.clone();
        is_modifying
    }
    fn set(&mut self, index: usize, value: &T) {
        self.mutable_slice()[index] = value.clone();
    }
    fn iter<'i>(&'i self) -> Self::ITER<'i> {
        self.shared_slice().iter()
    }

    // Ownership transfer constructors.
    fn from_shared_slice(slice: &'a [T]) -> Self {
        #[cfg(feature = "size_for_owned_only")]
        assert!(N.is_none());
        Self::Shared(slice)
    }
    fn from_mutable_slice(slice: &'a mut [T]) -> Self {
        #[cfg(feature = "size_for_owned_only")]
        assert!(N.is_none());
        Self::Mutable(slice)
    }
    fn from_array(array: [T; N.unwrap_or(0)]) -> Self {
        assert!(N.is_some());
        // \---> TODO consider const N: Option<usize>, or a custom enum.
        Self::Array(array)
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec(vector: Vec<T>) -> Self {
        #[cfg(feature = "size_for_owned_only")]
        assert!(N.is_none());
        Self::Vec(vector)
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn for_vec(vector: &'a mut Vec<T>) -> Self {
        Self::VecRef(vector)
    }

    // Constructors setting blank/default vaLues.
    /// Implemented only if T: Copy + Default.
    // Constructors setting blank/default vaLues.
    fn new_with_array() -> Self {
        #[cfg(feature = "size_for_owned_only")]
        assert!(N.is_none());
        Self::Array([T::default(); N.unwrap_or(0)])
    }
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn new_with_vec() -> Self {
        #[cfg(feature = "size_for_owned_only")]
        assert!(N.is_none());
        Self::from_vec(Vec::with_capacity(N.unwrap_or(0)))
        // @TODO populate
    }

    fn shared_slice<'s>(&'s self) -> &'s [T] {
        match &self {
            SliceStorage::Shared(slice) => slice,
            SliceStorage::Mutable(slice) => slice,
            SliceStorage::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => vec,
        }
    }

    fn mutable_slice<'s>(&'s mut self) -> &'s mut [T] {
        match self {
            SliceStorage::Shared(_) => {
                unimplemented!("Can't get a mutable slice from a shared slice.")
            }
            SliceStorage::Mutable(slice) => slice,
            SliceStorage::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => vec,
        }
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn mutable_vec<'s>(&'s mut self) -> &'s mut Vec<T> {
        match self {
            SliceStorage::Vec(vec) => vec,
            SliceStorage::VecRef(vec_ref) => *vec_ref,
            _ => {
                unimplemented!("Works for Vec and VecRef only.")
            }
        }
    }
}
impl<'s, T: 's + Clone, const N: Option<usize>> Clone for SliceStorage<'s, T, N>
where
    [(); N.unwrap_or(0)]:,
{
    /// Implemented for Array-backed and Vec-backed slice only.
    fn clone(&self) -> Self {
        match self {
            SliceStorage::Shared(_) | SliceStorage::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            SliceStorage::Array(array) => SliceStorage::Array(array.clone()),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => SliceStorage::Vec(vec.clone()),
        }
    }
}

impl<'s, T: 's + Clone + Copy + Default, const N: Option<usize>> crate::abstra::NewLike
    for SliceStorage<'s, T, N>
where
    [(); N.unwrap_or(0)]:,
{
    /// Implemented for Shared-backed, Array-backed and Vec-backed SliceStorage only.
    fn new_like(&self) -> Self {
        match self {
            SliceStorage::Shared(slice) => SliceStorage::Shared(slice),
            SliceStorage::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            SliceStorage::Array(_) => SliceStorage::Array([T::default(); N.unwrap_or(0)]),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(_) => SliceStorage::Vec(Vec::with_capacity(N.unwrap_or(0))),
        }
    }
}

pub type BoolSlice<'a, const N: Option<usize>> = SliceStorage<'a, bool, N>;
pub type ByteSlice<'a, const N: Option<usize>> = SliceStorage<'a, u8, N>;
