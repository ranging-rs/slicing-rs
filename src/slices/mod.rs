// @TODO consider name SliceStorageChoice
pub enum SliceStorageType {
    Shared,
    Mutable,
    Array,
    BoxArray,
    Vec,
    VecRef,
}

impl SliceStorageType {
    pub fn is_owned(&self) -> bool {
        use SliceStorageType::*;
        match self {
            Shared | Mutable | VecRef => false,
            Array | BoxArray | Vec => true,
        }
    }
}

/// Slice/array/vector-based container, with extra abstractions. You can use it
/// on its own.
/// The extra abstractions make it compatible with (limited) hash
/// set/hash map. Slice or hash set/hash then serve as pluggable in
/// (range-based) `ranging::byte_slice::ByteSliceBoolStorage` and in
/// implementations of `ranging::set::Set` and `ranging::map::Map`.
///
/// If array-based, the size is fixed at compile time through a const generic param `N`.
/// If slice-based or vec-based, its size can be any, as given at runtime. But for shared/mutable slice-based instances the size is fixed at instantiation.
/// Vec-based instances can be resized.
///
/// Param `N` indicates array size for SliceStorage::Array. It works together
/// with crate feature `size_for_array_only`.
///
/// If `N > 0`, that allows and reserves array storage in all variants of
///  `SliceStorage`. Then
///
/// -  if `size_for_array_only` is enabled, we allow `SliceStorage::Array`
/// variant only. We forbid (runtime) use of any other `SliceStorage` variants
///  (`SliceStorage::Shared`...) for non-zero `N`.
///  
/// That prevents us from
///  wasting (usually stack) unused memory. However, we have to type all
///  non-array variants as having `N = 0`, and hence we can't assign/pass those
///  non-array variants to an array variant.
///
/// - if `size_for_array_only` is disabled, we allow any variants of
///  `SliceStorage` (as applicable to the choice of `std` or `no_std`). That
///  gives code flexibility (as we can assign/pass any variants), but each
///  `SliceStorage` takes that array space - even if not used.
///
/// If `N = 0`, we either allow an array of size `0`, or we disable array variant.
///
/// - if `size_for_array_only` is enabled, we forbid (runtime) use of
///  `SliceStorage::Array` (with that `N = 0`). Use
/// slice/vector-based variants of `SliceStorage` (`SliceStorage::Shared`...,
///  `SliceStorage::Vec`...) instead.
///
/// - if `size_for_array_only` is disabled, we allow array storage with size `0`
/// (for `SliceStorage::Array`) and we allow all other variants (as applicable
/// according to `std` or `no_std`), too.
///
/// # Naming convention for methods:
/// - `as_***()` means conversion (sharing), but not a copy
/// - `to_***()` means a copy.
///
/// # Array size `N` and zero sized arrays
/// Generally,
/// - test your crate with feature `size_for_array_only` enabled, and
/// - don't hard code any non-zero `N` (unless sure), but have it come from
///  the client. Those two rules will guide you to:
/// - use given non-zero `N` only for array-based purposes (often on stack, or in `non_std`). Otherwise use `SliceStorage<T, 0>` for pathways based on a slice/vector.
/// - use `as_non_array_***` methods whenever possible.
///
/// `Slice` (and it implementations) don't have `const N: usize`. That
///  would allow  granular (per generic type instance) intent as to whether the
///  specific type (for its chosen `N`) allows arrays of the given size
///  (whether zero or not), or whether it disables its array-based variant.
/// However, that required extra bounds like `where [(); N.unwrap_or(0)]:`,
/// not only in the `trait`s and `struct`s & `impl`, but also in any client
/// code! Very unergonomic.
/// Hence we have `const N: usize` instead. That means that specific types can't
/// control/vary in whether they disable any array or whether they enable empty
/// arrays. Such difference is possible only globally with crate features
///  `allow_empty_arrays` & `disable_empty_arrays`. Those two features are
/// mutually exclusive. If none of them is set, then with `N = 0` you can use
/// both empty arrays (array-based variant) and, of course, any non-array
/// variants (as applicable according to your `std or `non_std`).
pub trait Slice<'a, T: 'a + Clone + PartialEq, const N: usize>
where
    Self: 'a,
{
    type ITER<'i>: Iterator<Item = &'i T> = core::slice::Iter<'i, T> where T: 'i, Self: 'i;

    /// Like Self, but with size 0. `NARR` means NON_ARRAY. It serves for
    /// conversion functions that return or accept the same a Slice
    /// implementation type as `Self` but with size 0.
    /// There's no way, and no need, to correlate `NARR` and `Self` here any
    /// closer (even though those types are related). It's the semantics/
    /// convention that matters.
    type NARR: Slice<'a, T, 0>;

    fn get(&self, index: usize) -> T;
    /// Set the value. Return true if this value was not present. (Based on std::collections::HashSet.)
    fn check_and_set(&mut self, index: usize, value: &T) -> bool;
    /// Set the value.
    fn set(&mut self, index: usize, value: &T);
    fn iter<'s>(&'s self) -> Self::ITER<'s>;

    // Constructor that doesn't transfer an array, but it transfers its slice.
    fn from_shared(slice: &'a [T]) -> Self;
    // Constructor that doesn't transfer an array, but it transfers its slice.
    fn from_mutable(slice: &'a mut [T]) -> Self;
    /// Array ownership transfer constructor.
    fn from_array(array: [T; N]) -> Self;

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec(vector: Vec<T>) -> Self;

    /// Non-transfer constructor referring to a given `vector`. It transfers
    /// ownership of the (mutable) reference itself.
    /// The only benefit of this function, as compared to `from_mutable_slice`,
    /// is that we can call `mutable_vec` on this instance.
    /// This function doesn't need a shared/immutable alternative - for that
    /// use simple `from_shared`.
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec_ref(vector: &'a mut Vec<T>) -> Self;

    // @TODO to a separate trait:
    fn from_default(storage_type: SliceStorageType) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    // Populating constructors - creating an instance that owns the data.
    // @TODO And/Or:
    // from_value(storage_type) for Array|ArrayBox, and
    // from_value_to_vec(size), OR
    //       \\\ <-- Good for auto-complete. (Unlike vec_from_value(...))
    //       But vec is more common than array. Hence:
    // from_value_to_array(value) and
    // from_value_to_box_array(value)
    // AND
    // from_value(value, size)  -> owned vec
    // --> @TODO add to README.md.
    fn from_value(storage_type: SliceStorageType, size: usize) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    // @TODO SliceStorageType
    fn from_iter() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    // from_iter_to_vec(storage_type, size)
    fn from_call() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    // @TODO move/join with the above:
    // Constructors setting blank/default vaLues.
    /// Implemented only if T: Copy + Default.
    fn new_with_array() -> Self;

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    // @TODO move/join with the above:
    /// Implemented only if T: Default.
    fn new_with_vec(size: usize) -> Self;

    // Reference/link-based constructors. Ever needed? Couldn't we just pass a shared/mutable reference to the existing Slice instance?
    /*fn to_shared_based<'s>(&'s self) -> Self
    fn to_mutable_based<'s>(&'s mut self) -> Self
    */

    // Copy constructors.
    /// Copy to a new array and create an instance with it.
    fn to_array_based(&self) -> Self;

    // @TODO Would `copy_to_vec_based` be a better name?
    /// Copy to a new vec and create an instance with it.
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn to_vec_based(&self) -> Self;

    // Again, any need for the following? Couldn't we just pass a &mut to the existing (vec-based) Slice instance?
    // fn to_vec_ref_based(&mut self) -> Self

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn to_non_array_vec_based(&self) -> Self::NARR;

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
pub enum SliceStorage<'a, T: 'a, const N: usize> {
    Shared(&'a [T]),
    Mutable(&'a mut [T]),
    /// Owned array. Suggested for stack & `no_std`.
    Array([T; N]),

    // @TODO no_std alloc-based Box<[T; N]> ???
    /// Owned vector. For `std` only.
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    Vec(Vec<T>),

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    VecRef(&'a mut Vec<T>),
}

// TODO If we ever need this for non-Copy, then split this, and for non-Copy make `new_with_array()` and `to_array_based` panic!().
impl<'a, T: 'a + Copy + PartialEq + Default, const N: usize> Slice<'a, T, N>
    for SliceStorage<'a, T, N>
{
    type ITER<'i> = core::slice::Iter<'i, T>
    where T: 'i, Self: 'i;

    type NARR = SliceStorage<'a, T, 0>;

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
    fn from_shared(slice: &'a [T]) -> Self {
        #[cfg(feature = "size_for_array_only")]
        assert!(N.is_none());
        Self::Shared(slice)
    }
    fn from_mutable(slice: &'a mut [T]) -> Self {
        #[cfg(feature = "size_for_array_only")]
        assert!(N.is_none());
        Self::Mutable(slice)
    }
    fn from_array(array: [T; N]) -> Self {
        #[cfg(feature = "disable_empty_arrays")]
        assert!(N > 0);
        Self::Array(array)
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec(vector: Vec<T>) -> Self {
        #[cfg(feature = "size_for_array_only")]
        assert!(N.is_none());
        Self::Vec(vector)
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn from_vec_ref(vector: &'a mut Vec<T>) -> Self {
        Self::VecRef(vector)
    }

    // Constructors setting blank/default vaLues.
    /// Implemented only if T: Copy + Default.
    // Constructors setting blank/default vaLues.
    fn new_with_array() -> Self {
        #[cfg(feature = "size_for_array_only")]
        assert!(N.is_none());
        Self::Array([T::default(); N])
    }
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn new_with_vec(size: usize) -> Self {
        #[cfg(feature = "size_for_array_only")]
        assert!(N.is_none());
        Self::from_vec(Vec::with_capacity(size))
        // @TODO populate
    }

    fn to_array_based(&self) -> Self {
        #[cfg(feature = "disable_empty_arrays")]
        assert!(N > 0);
        match self {
            SliceStorage::Array(from) => {
                let to = *from;
                SliceStorage::Array(to)
            }
            SliceStorage::Shared(slice) => {
                let mut to = [T::default(); N];
                to.copy_from_slice(*slice);
                SliceStorage::Array(to)
            }
            SliceStorage::Mutable(slice) => {
                let mut to = [T::default(); N];
                to.copy_from_slice(*slice);
                SliceStorage::Array(to)
            }
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => {
                let mut to = [T::default(); N];
                to.copy_from_slice(vec);
                SliceStorage::Array(to)
            }
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::VecRef(vec_ref) => {
                let mut to = [T::default(); N];
                to.copy_from_slice(*vec_ref);
                SliceStorage::Array(to)
            }
        }
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn to_vec_based(&self) -> Self {
        todo!()
    }

    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    fn to_non_array_vec_based(&self) -> Self::NARR {
        let v: Vec<T>;
        if let Self::Mutable(mutable_slice) = self {
            v = Vec::from_iter(mutable_slice.iter().cloned());
        } else {
            let slice = match self {
                Self::Array(array) => array,
                Self::Shared(shared_slice) => *shared_slice,
                #[cfg(all(not(feature = "no_std"), feature = "std"))]
                Self::Vec(vec) => vec,
                #[cfg(all(not(feature = "no_std"), feature = "std"))]
                Self::VecRef(vec_ref) => *vec_ref,
                _ => unreachable!(),
            };
            v = Vec::from_iter(slice.iter().cloned());
        }
        Self::NARR::Vec(v)
    }

    // Accessors
    fn shared_slice<'s>(&'s self) -> &'s [T] {
        match &self {
            SliceStorage::Shared(slice) => slice,
            SliceStorage::Mutable(slice) => slice,
            SliceStorage::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => vec,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::VecRef(vec_ref) => *vec_ref,
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
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::VecRef(vec_ref) => *vec_ref,
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
impl<'s, T: 's + Clone, const N: usize> Clone for SliceStorage<'s, T, N> {
    /// Implemented for Array-backed and Vec-backed SliceStorage only. For Vec (mutable) reference-backed SliceStorage this creates a new, owned Vec-based instance.
    fn clone(&self) -> Self {
        match self {
            SliceStorage::Shared(_) => {
                unimplemented!("Don't clone a shared slice-backed SliceStorage. Instead, pass a shared reference to it.")
            }
            SliceStorage::Mutable(_) => {
                unimplemented!("Can't clone a mutable slice.")
            }
            SliceStorage::Array(array) => SliceStorage::Array(array.clone()),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => SliceStorage::Vec(vec.clone()),
            // Can't clone a mutable reference. Clone the vector itself.
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::VecRef(vec_ref) => SliceStorage::Vec((*vec_ref).clone()),
        }
    }
}

impl<'s, T: 's + Clone + Copy + Default, const N: usize> crate::abstra::NewLike
    for SliceStorage<'s, T, N>
{
    /// Implemented for Shared-backed, Array-backed and Vec-backed SliceStorage only.
    fn new_like(&self) -> Self {
        match self {
            SliceStorage::Shared(slice) => SliceStorage::Shared(slice),
            SliceStorage::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            SliceStorage::Array(_) => SliceStorage::Array([T::default(); N]),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::Vec(vec) => SliceStorage::Vec(Vec::with_capacity(vec.len())),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            SliceStorage::VecRef(vec_ref) => {
                SliceStorage::Vec(Vec::with_capacity((*vec_ref).len()))
            }
        }
    }
}

pub type BoolSlice<'a, const N: usize> = SliceStorage<'a, bool, N>;
pub type ByteSlice<'a, const N: usize> = SliceStorage<'a, u8, N>;
