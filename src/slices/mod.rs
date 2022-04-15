#[cfg(any(feature = "no_std_box", feature = "no_std_vec"))]
extern crate alloc;
#[cfg(feature = "no_std_box")]
use alloc::boxed::Box;
#[cfg(feature = "no_std_vec")]
use alloc::vec::Vec;

macro_rules! slice_trait {
    ($trait_name:ident) => {
        type ITER<'i>: Iterator<Item = &'i T> = core::slice::Iter<'i, T> where T: 'i, Self: 'i;

        /// Like Self, but with size 0. `NARR` means NON_ARRAY. It serves for
        /// conversion functions that return or accept the same Slice
        /// implementation type as `Self` but with size 0.
        /// There's no way, and no need, to correlate `NARR` and `Self` here any
        /// closer (even though those types are related). It's the semantics/
        /// convention that matters.
        type NARR: $trait_name<'a, T, 0>;

        fn get(&self, index: usize) -> T;
        /// Set the value. Return true if this value was not present. (Like std::collections::HashSet.)
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

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn from_vec(vector: Vec<T>) -> Self;

        /// Non-transfer constructor mutably referring to a given `vector`. It transfers
        /// ownership of the (mutable) reference itself.
        /// The only benefit of this function, as compared to `from_mutable_slice`,
        /// is that we can call `mutable_vec` on this instance.
        /// This function doesn't need a shared/immutable alternative - for that
        /// use simple `from_shared`.
        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn from_vec_ref(vector: &'a mut Vec<T>) -> Self;

        // Populating constructors - creating an instance that owns the data.
        fn from_value(value: &'a T, size: usize, storage_type: SliceBackedChoice) -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        fn from_value_to_array(value: &'a T) -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        fn from_value_to_box_array(value: &'a T) -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        fn from_value_to_vec(value: &'a T, size: usize) -> Self
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

        // Reference/link-based constructors. Ever needed? Couldn't we just pass a shared/mutable reference to the existing Slice instance?
        /*fn to_shared_based<'s>(&'s self) -> Self
        fn to_mutable_based<'s>(&'s mut self) -> Self
        */

        // Copy constructors.
        // @TODO Would `copy_to_vec_based` be a better name?
        /// Copy to a new vec and create an instance with it.
        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn to_vec_based(&self) -> Self;

        // Again, any need for the following? Couldn't we just pass a &mut to the existing (vec-based) Slice instance?
        // fn to_vec_ref_based(&mut self) -> Self

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn to_non_array_vec_based(&self) -> Self::NARR;

        // Accessors
        fn shared_slice(&self) -> &[T];
        /// Implemented for all except for Shared-based slice.
        fn mutable_slice<'s>(&'s mut self) -> &'s mut [T];
        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn mutable_vec<'s>(&'s mut self) -> &'s mut Vec<T>;
    };
}

macro_rules! slice_trait_default {
    () => {
        /// Copy to a new array and create an instance with it.
        fn to_array_based(&self) -> Self;

        // @TODO to a separate trait - for Default only:
        /// Param `size` is used only if `storage_type == SliceBackedChoice::Vec`.
        /// Param `storage_type` can be only for "owned" choices (Array/BoxArray/Vec).
        fn from_default(size: usize, storage_type: SliceBackedChoice) -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        fn from_default_to_array() -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
        fn from_default_to_box_array() -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn from_default_to_vec(size: usize) -> Self
        where
            Self: Sized,
        {
            todo!()
        }

        // Constructors setting blank/default values.
        /// Implemented only if T: Copy. TODO playground-based for Clone.
        fn new_with_array() -> Self
        where
            Self: Sized,
        {
            todo!()
        }
        #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
        fn new_with_box_array() -> Self
        where
            Self: Sized,
        {
            todo!()
        }

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        /// Implemented only if T: Default.
        fn new_with_vec(size: usize) -> Self
        where
            Self: Sized,
        {
            todo!()
        }
    };
}

//@TODO in `Set` module/structs: `no_std` friendly:
//use alloc::collections::BTreeMap;

/// Like `SliceClone`, but for `Copy` types.
pub trait Slice<'a, T: 'a + Clone + Copy + PartialEq, const N: usize>
where
    Self: 'a,
{
    slice_trait!(Slice);
}

pub trait SliceDefault<'a, T: 'a + Clone + Copy + PartialEq + Default, const N: usize>
where
    Self: 'a,
{
    slice_trait!(SliceDefault);
    slice_trait_default!();
}

/// Slice/array/vector-based container, with extra abstractions. You can use it
/// on its own.
/// The extra abstractions also make it compatible with (limited and adapted)
/// hash set/hash map. Slice or hash set/hash then serve as pluggable in
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
/// `Slice` (and it implementations) have `const N: usize` and not `const N: Option<usize>`. The later
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
///
/// If `T` is `Copy`, use `Slice` instead. See also `SliceStorageClone` and
/// `SliceStorage` for reasoning on why they are named so.
pub trait SliceClone<'a, T: 'a + Clone + PartialEq, const N: usize>
where
    Self: 'a,
{
    slice_trait!(SliceClone);
}

pub trait SliceDefaultClone<'a, T: 'a + Clone + PartialEq + Default, const N: usize>
where
    Self: 'a,
{
    slice_trait!(SliceDefaultClone);
    slice_trait_default!();
}

#[derive(Debug, PartialEq)]
pub enum SliceBackedChoice {
    Shared,
    Mutable,
    Array,
    #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
    BoxArray,
    // @TODO with vectors only
    Vec,
    VecRef,
}

impl SliceBackedChoice {
    pub fn is_owned(&self) -> bool {
        use SliceBackedChoice::*;
        match self {
            Shared | Mutable | VecRef => false,
            // @TODO with vectors only
            Array => true,
            #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
            BoxArray => true,
            Vec => true,
        }
    }
}

/// Like `SliceClone`, but backed by a native slice or similar.
pub trait SliceBackedClone<'a, T: 'a + Clone + PartialEq, const N: usize>:
    SliceClone<'a, T, N>
where
    Self: 'a,
{
}

macro_rules! slice_storage_enum {
    ($enum_name:ident, $($item_bounds:tt)+) => {
        #[derive(Debug)]
        pub enum $enum_name<'a, T: 'a + $($item_bounds)+, const N: usize> {
            Shared(&'a [T]),
            Mutable(&'a mut [T]),

            /// Owned array. Suggested for stack & `no_std`.
            Array([T; N]),
            /// Owned boxed array.
            #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
            BoxArray(Box<[T; N]>),

            /// Owned vector.
            #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
            Vec(Vec<T>),

            #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
            VecRef(&'a mut Vec<T>),
        }
    };
}

/// Abstracted slice storage/access.
/// Const generic param `N` is used by `SliceStorage::Array` and `SliceStorage::BoxArray` invariants only. However, it makes all variants consume space. Hence `N > 0` is suggested primarily for no-heap or frequent instantiation on stack and for small sizes (`N`).
/// If you run with heap, and you have infrequent instantiation or large sizes (`N`), suggest passing 0 for `N`, and use `SliceStorage::Vec` or `SliceStorage::BoxArray` instead.
///
/// Why don't we call this to `SliceStorageCopy` instead of `SliceStorage` and why don't we
/// rename the existing `SliceStorageClone`
/// to `SliceStorage`? It could lead to laziness/
/// not noticing/forgetting to use `SliceStorageCopy` whenever possible.
/// Especially so because then any `Copy` type could be stored in either
/// `SliceStorage` or `SliceStorageCopy`. Storing `Copy` in Clone-friendly
/// storage would work, but it would be less efficient than storing it in a
/// specialized `Copy` storage. Even more so with default values (which, if
/// primitive and if stored in specialized `Copy` storage, could be
/// optimized away - so the
/// virtual memory wouldn't be used until written to it later).
/// However, if we have `SliceStorage` work for `Copy` only, then it's unlikely
/// that anyone would use it for `Clone` types (even though they could). And we
/// have `SliceStorageClone` for `Clone` types.
//pub enum SliceStorage<'a, T: 'a, const N: usize>
slice_storage_enum!(SliceStorage, Clone + Copy);
slice_storage_enum!(SliceStorageClone, Clone);
slice_storage_enum!(SliceStorageDefault, Clone + Copy + Default);
slice_storage_enum!(SliceStorageDefaultClone, Clone + Default);

macro_rules! slice_storage_impl {
    ($enum_name:ident) => {
        type ITER<'i> = core::slice::Iter<'i, T>
                                                where T: 'i, Self: 'i;

        type NARR = $enum_name<'a, T, 0>;

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

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn from_vec(vector: Vec<T>) -> Self {
            #[cfg(feature = "size_for_array_only")]
            assert!(N.is_none());
            Self::Vec(vector)
        }

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn from_vec_ref(vector: &'a mut Vec<T>) -> Self {
            Self::VecRef(vector)
        }

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn to_vec_based(&self) -> Self {
            todo!()
        }

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn to_non_array_vec_based(&self) -> Self::NARR {
            let v: Vec<T>;
            if let Self::Mutable(mutable_slice) = self {
                v = Vec::from_iter(mutable_slice.iter().cloned());
            } else {
                let slice = match self {
                    Self::Array(array) => array,
                    #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
                    Self::BoxArray(boxed_array) => &(**boxed_array),
                    Self::Shared(shared_slice) => *shared_slice,
                    #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                    Self::Vec(vec) => vec,
                    #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                    Self::VecRef(vec_ref) => *vec_ref,
                    _ => unreachable!(),
                };
                v = Vec::from_iter(slice.iter().cloned());
            }
            Self::NARR::Vec(v)
        }

        // Accessors
        fn shared_slice(&self) -> &[T] {
            match &self {
                Self::Shared(slice) => slice,
                Self::Mutable(slice) => slice,
                Self::Array(array) => array,
                #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
                Self::BoxArray(boxed_array) => &(**boxed_array),
                #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                Self::Vec(vec) => vec,
                #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                Self::VecRef(vec_ref) => *vec_ref,
            }
        }

        fn mutable_slice<'s>(&'s mut self) -> &'s mut [T] {
            match self {
                Self::Shared(_) => {
                    unimplemented!("Can't get a mutable slice from a shared slice.")
                }
                Self::Mutable(slice) => slice,
                Self::Array(array) => array,
                #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
                Self::BoxArray(boxed_array) => &mut (**boxed_array),
                #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                Self::Vec(vec) => vec,
                #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                Self::VecRef(vec_ref) => *vec_ref,
            }
        }

        #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
        fn mutable_vec<'s>(&'s mut self) -> &'s mut Vec<T> {
            match self {
                Self::Vec(vec) => vec,
                Self::VecRef(vec_ref) => *vec_ref,
                _ => {
                    unimplemented!("Works for Vec and VecRef only.")
                }
            }
        }
    };
}

impl<'a, T: 'a + Clone + Copy + PartialEq, const N: usize> Slice<'a, T, N>
    for SliceStorage<'a, T, N>
{
    slice_storage_impl!(SliceStorage);
}
impl<'a, T: 'a + Clone + PartialEq, const N: usize> SliceClone<'a, T, N>
    for SliceStorageClone<'a, T, N>
{
    slice_storage_impl!(SliceStorageClone);
}

// Following functions are in pairs, used as alternative implementation parts
// passed to slice_storage_default_impl macro.
fn copy_array<T: Clone + Copy, const N: usize>(from: &[T; N]) -> [T; N] {
    *from
}
fn clone_array<T: Clone, const N: usize>(from: &[T; N]) -> [T; N] {
    from.clone()
}
fn copy_array_default<T: Clone + Copy + Default, const N: usize>() -> [T; N] {
    [T::default(); N]
}
// @TODO optimize - later
fn clone_array_default<T: Clone + Default, const N: usize>() -> [T; N] {
    core::array::from_fn(|_| T::default())
}
macro_rules! slice_storage_default_impl {
    ($copy_or_clone_from_slice: ident, $copy_or_clone_array: ident, $copy_or_clone_default: ident) => {
        fn to_array_based(&self) -> Self {
            #[cfg(feature = "disable_empty_arrays")]
            assert!(N > 0);
            match self {
                Self::Array(from) => {
                    let to = $copy_or_clone_array(from);
                    Self::Array(to)
                }
                #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
                Self::BoxArray(from) => {
                    let to = $copy_or_clone_array(&(**from));
                    Self::Array(to)
                }
                Self::Shared(slice) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(*slice);
                    Self::Array(to)
                }
                Self::Mutable(slice) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(*slice);
                    Self::Array(to)
                }
                #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                Self::Vec(vec) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(vec);
                    Self::Array(to)
                }
                #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
                Self::VecRef(vec_ref) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(*vec_ref);
                    Self::Array(to)
                }
            }
        }
    };
}

impl<'a, T: 'a + Clone + Copy + PartialEq + Default, const N: usize> SliceDefault<'a, T, N>
    for SliceStorageDefault<'a, T, N>
{
    slice_storage_impl!(SliceStorageDefault);
    slice_storage_default_impl!(copy_from_slice, copy_array, copy_array_default);
}
impl<'a, T: 'a + Clone + PartialEq + Default, const N: usize> SliceDefaultClone<'a, T, N>
    for SliceStorageDefaultClone<'a, T, N>
{
    slice_storage_impl!(SliceStorageDefaultClone);
    slice_storage_default_impl!(clone_from_slice, clone_array, clone_array_default);
}

/*
   //@TODO
   // Constructors setting blank/default vaLues.
   /// Implemented only if T: Copy + Default.
   // Constructors setting blank/default vaLues.
   fn new_with_array() -> Self {
       #[cfg(feature = "size_for_array_only")]
       assert!(N > 0);
       Self::Array([T::default(); N])
   }
   #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
   fn new_with_box_array() -> Self {
       #[cfg(feature = "size_for_array_only")]
       assert!(N > 0);
       Self::BoxArray(Box::new([T::default(); N]))
   }
   #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
   fn new_with_vec(size: usize) -> Self {
       #[cfg(feature = "size_for_array_only")]
       assert!(N > 0);
       Self::from_vec(Vec::with_capacity(size))
       // @TODO populate
   }
*/

macro_rules! slice_storage_impl_clone {
() => {
    /// Implemented for Array-backed and Vec-backed SliceStorage only. For Vec (mutable) reference-backed SliceStorage this creates a new, owned Vec-based instance.
    fn clone(&self) -> Self {
        match self {
            Self::Shared(_) => {
                unimplemented!("Don't clone a shared slice-backed SliceStorage. Instead, pass a shared reference to it.")
            }
            Self::Mutable(_) => {
                unimplemented!("Can't clone a mutable slice.")
            }
            Self::Array(array) => Self::Array(array.clone()),
            #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
            Self::BoxArray(boxed_array) => Self::BoxArray(boxed_array.clone()),
            #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
            Self::Vec(vec) => Self::Vec(vec.clone()),
            // Can't clone a mutable reference. Clone the vector itself.
            #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
            Self::VecRef(vec_ref) => Self::Vec((*vec_ref).clone()),
        }
    }
};
}

impl<'s, T: 's + Clone + Copy, const N: usize> Clone for SliceStorage<'s, T, N> {
    slice_storage_impl_clone!();
}
impl<'s, T: 's + Clone, const N: usize> Clone for SliceStorageClone<'s, T, N> {
    slice_storage_impl_clone!();
}
impl<'a, T: 'a + Clone + Copy + PartialEq + Default, const N: usize> Clone
    for SliceStorageDefault<'a, T, N>
{
    slice_storage_impl_clone!();
}
impl<'a, T: 'a + Clone + PartialEq + Default, const N: usize> Clone
    for SliceStorageDefaultClone<'a, T, N>
{
    slice_storage_impl_clone!();
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
            #[cfg(any(not(feature = "no_std"), feature = "no_std_box"))]
            SliceStorage::BoxArray(_) => SliceStorage::BoxArray(Box::new([T::default(); N])),
            #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
            SliceStorage::Vec(vec) => SliceStorage::Vec(Vec::with_capacity(vec.len())),
            #[cfg(any(not(feature = "no_std"), feature = "no_std_vec"))]
            SliceStorage::VecRef(vec_ref) => {
                SliceStorage::Vec(Vec::with_capacity((*vec_ref).len()))
            }
        }
    }
}

pub type BoolSlice<'a, const N: usize> = SliceStorageDefault<'a, bool, N>;
pub type ByteSlice<'a, const N: usize> = SliceStorageDefault<'a, u8, N>;
