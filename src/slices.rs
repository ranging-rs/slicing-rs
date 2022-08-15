#[cfg(feature = "no_std_heap")]
extern crate alloc;
#[cfg(feature = "no_std_heap")]
use alloc::vec::Vec;

use core::array;

/// This requires commas between any matching branches, even if the previous
/// match expression (result right of `=>`) is a block `{...}`. (It can't accept
/// a trailing comma.)
#[macro_export]
macro_rules! match_cfg {
    (
        $value_to_match:expr,
        $(
            $(#[ $meta_or_docs:meta ])*
            // There must be no #[meta], neither any /// docs, between the
            // following ones and the `match` pattern.
            $(~[ heap $heap_delimiter:tt ])?
            $(~[ std $std_delimiter:tt ])?

            $match_pattern:pat => $match_expression:expr
        ),+
    ) => {
        match $value_to_match {
            $(
                $(
                    #[$meta_or_docs]
                )*
                $(
                    #[cfg(any(not(feature = "no_std"), feature = "no_std_heap"))]
                    ${ignore(heap_delimiter)}
                )?
                $(
                    #[cfg(not(feature = "no_std"))]
                    ${ignore(std_delimiter)}
                )?

                $match_pattern =>  $match_expression
            ),+
        }
    }
}

#[macro_export]
macro_rules! enum_cfg {
    (
        (
            $(
                $header_part:tt
            )+
        )$(,)?
        $(
            $(#[ $meta_or_docs:meta ])*
            // There must be no #[meta], neither any /// docs, between the
            // following ones and the enum variant pattern.
            $(~[ heap $heap_delimiter:tt ])?
            $(~[ std $std_delimiter:tt ])?

            $variant:ident
            $(
                (
                    $( $tuple_fields: tt )+
                )
            )?
            $(
                {
                    $( $named_fields: tt )+
                }
            )?
        ),+

    ) => {
        $( $header_part )+
        {
            $(
                $(
                    #[$meta_or_docs]
                )*
                $(
                    #[cfg(any(not(feature = "no_std"), feature = "no_std_heap"))]
                    ${ignore(heap_delimiter)}
                )?
                $(
                    #[cfg(not(feature = "no_std"))]
                    ${ignore(std_delimiter)}
                )?

                $variant
                $(
                    (
                        $( $tuple_fields )+
                    )
                )?
                $(
                    {
                        $( $named_fields )+
                    }
                )?
            ),+
        }
    }
}

/// Conditionally compile - only if heap is supported. Do not group statements!
/// Instead, either use a separate macro invocation per each
/// statement/definition, or put a sequence of statements inside an inner block
/// `{...`}. In places where we can't use this macro, such as conditionally
/// compiling branches of a `match` statement, we use
/// ```
/// #[cfg(any(not(feature = "no_std"), feature = "no_std_heap"))] // with_heap
/// {} // a statement here
/// ```
/// including the comment `//with_heap`, so that we can search for both usages
/// easily.
#[macro_export]
macro_rules! with_heap {
    (
        $($item:tt)*
    ) => {
        #[cfg(any(not(feature = "no_std"), feature = "no_std_heap"))]
        $($item)*
    }
}

#[macro_export]
macro_rules! with_std {
    (
        $($item:tt)*
    ) => {
        #[cfg(not(feature = "no_std"))]
        $($item)*
    }
}

/// Helper that generates code shared between various Slice traits.
/// Non-hygienic, of course.
// Purpose of separation from slice_trait_with_narr_size: When
// `disable_empty_arrays` feature is enabled, this makes `NARR` not be an array
// with 0 items (which would fail), but with one item.
macro_rules! slice_trait {
    ($trait_name:ident) => {
        #[cfg(not(feature = "disable_empty_arrays"))]
        slice_trait_with_narr_size!($trait_name, 0);
        #[cfg(feature = "disable_empty_arrays")]
        slice_trait_with_narr_size!($trait_name, 1);
    };
}

/// Workaround Rust's limitations on NARR's size parameter and its bounds. See
/// https://github.com/ranging-rs/slicing-rs/issues/1
macro_rules! slice_trait_with_narr_size {
    ($trait_name:ident, $narr_size:expr) => {
        type ITER<'i>: Iterator<Item = &'i T> = core::slice::Iter<'i, T> where T: 'i, Self: 'i;

        with_heap!{
            /// Like Self, but with size 0. `NARR` means NON_ARRAY. It serves
            /// for conversion functions that return or accept the same Slice
            /// implementation type as `Self` but with size 0. There's no way,
            /// and no need, to correlate `NARR` and `Self` here any closer
            /// (even though those types are related). It's the semantics/
            /// convention that matters. (The actual implementation type will
            /// define `NARR` as itself but with size 0.)
            type NARR: $trait_name<'a, T, $narr_size>;
        }

        fn get(&self, index: usize) -> T;
        /// Set the value. Return true if this value was not present. (Like
        /// std::collections::HashSet.)
        fn check_and_set(&mut self, index: usize, value: &T) -> bool;
        /// Set the value.
        fn set(&mut self, index: usize, value: &T);
        fn iter<'s>(&'s self) -> Self::ITER<'s>;

        // Constructor that doesn't transfer an array, but it transfers its
        // slice.
        fn from_shared(slice: &'a [T]) -> Self;
        // Constructor that doesn't transfer an array, but it transfers its
        // slice.
        fn from_mutable(slice: &'a mut [T]) -> Self;
        /// Array ownership transfer constructor.
        fn from_array(array: [T; N]) -> Self;

        with_heap! {
            fn from_vec(vector: Vec<T>) -> Self;
        }

        with_heap! {
            fn from_vec_with_capacity(capacity: usize) -> Self
            where
                Self: Sized,
            {
                Self::from_vec(Vec::with_capacity(capacity))
            }
        }

        with_heap! {
            fn from_vec_new() -> Self
            where
                Self: Sized,
            {
                Self::from_vec(Vec::new())
            }
        }

        with_heap! {
            /// Non-transfer constructor mutably referring to a given `vector`.
            /// It transfers ownership of the (mutable) reference itself. The
            /// only benefit of this function, as compared to
            /// `from_mutable_slice`, is that we can call `mutable_vec` on this
            /// instance. This function doesn't need a shared/immutable
            /// alternative - for that use simple `from_shared`.
            fn from_vec_ref(vector: &'a mut Vec<T>) -> Self;
        }

        // Populating constructors - creating an instance that owns the data.
        fn from_value(value_ref: &'a T, size: usize, storage_type: SliceBackedChoice) -> Self
        where
            Self: Sized,
        {
            match_cfg! { storage_type,
                ~[heap~]
                SliceBackedChoice::Array => Self::from_value_to_array(value_ref),
                // @TODO test this - duplicate pattern?!
                ~[std~]
                SliceBackedChoice::Array | SliceBackedChoice::Array=> Self::from_value_to_array(value_ref),
                _ => unimplemented!("Never")
            }
        }
        fn from_value_to_array(value: &'a T) -> Self;
        with_heap! {
            fn from_value_to_vec(value: &'a T, size: usize) -> Self;
        }

        fn from_iter_to(
            iter: impl Iterator<Item = T>,
            size: usize,
            storage_type: SliceBackedChoice,
        ) -> Self
        where
            Self: Sized,
        {
            match_cfg! {storage_type,
                SliceBackedChoice::Array => Self::from_iter_to_array(iter),
                ~[heap~]
                SliceBackedChoice::Vec => Self::from_iter_to_vec(iter),
                _ => unimplemented!("Never")
            }
        }
        fn from_iter_to_array(iter: impl Iterator<Item = T>) -> Self;
        with_heap! {
            fn from_iter_to_vec(iter: impl Iterator<Item = T>) -> Self;
        }

        fn from_fn_to(f: impl FnMut() -> T, size: usize, storage_type: SliceBackedChoice) -> Self
        where
            Self: Sized,
        {
            match_cfg! {storage_type,
                SliceBackedChoice::Array => Self::from_fn_to_array(f),
                ~[heap~]
                SliceBackedChoice::Vec => Self::from_fn_to_vec(f, size),
                _ => unimplemented!("Never")
            }
        }
        fn from_fn_to_array(f: impl FnMut() -> T) -> Self;
        with_heap! {
            fn from_fn_to_vec(f: impl FnMut() -> T, size: usize) -> Self;
        }

        with_heap! {
            /// Replace with the same or a new vec-based instance.
            fn to_vec_based(self) -> Self;
        }

        with_heap! {
            // Copy constructor.
            fn to_non_array_vec_based(&self) -> Self::NARR;
        }

        with_heap! {
            // Copy constructor. Not an "as_..." or transformation constructor,
            // because `Self` is non-zero-sized. Otherwise we'd be wasting
            // memory occupied by `Self`.
            fn from_non_array_vec_based(from: &Self::NARR,
                as_choice: &SliceBackedChoice) -> Self;
        }

        // Accessors
        fn shared_slice(&self) -> &[T];
        /// Implemented for all except for Shared-based slice.
        fn mutable_slice<'s>(&'s mut self) -> &'s mut [T];
        with_heap! {
            fn mutable_vec<'s>(&'s mut self) -> &'s mut Vec<T>;
        }
    };
}

macro_rules! slice_trait_default {
    () => {
        /// Copy to a new array and create an instance with it.
        fn to_array_based(&self) -> Self;

        /// Param `size` is used only if `storage_type ==
        /// SliceBackedChoice::Vec`. Param `storage_type` can be only for
        /// "owned" choices (Array/BoxArray/Vec).
        fn from_default(size: usize, storage_type: SliceBackedChoice) -> Self
        where
            Self: Sized,
        {
            match_cfg! {storage_type,
                SliceBackedChoice::Array => Self::from_default_to_array(),
                ~[heap~]
                SliceBackedChoice::Vec => Self::from_default_to_vec(size),
                _ => unimplemented!("Never")
            }
        }
        fn from_default_to_array() -> Self;
        with_heap! {
            fn from_default_to_vec(size: usize) -> Self;
        }
    };
}

//@TODO in `Set` module/structs: `no_std` friendly: use
//alloc::collections::BTreeMap;

/** Check that given `N` is non-zero if crate feature `disable_empty_arrays` is
 *  enabled. Otherwise panic.
 *  Return array_size.
 **/
#[allow(unused_variables)]
pub const fn check_empty_array_size(array_size: usize) -> usize {
    #[cfg(feature = "disable_empty_arrays")]
    if array_size == 0 {
        panic!("Empty arrays are not allowed, due to disable_empty_arrays.");
    };
    array_size
}

/// Like `SliceClone`, but for `Copy` types.
pub trait Slice<'a, T: 'a + Clone + Copy + PartialEq, const N: usize>
where
    Self: 'a,
    [(); check_empty_array_size(N)]:,
{
    slice_trait!(Slice);
}

pub trait CollectTo<'a>
where
    <Self as CollectTo<'a>>::Item: 'a,
{
    type Item: Clone + Copy + PartialEq;
    fn collect_to<S: Slice<'a, Self::Item, N>, const N: usize>(
        self,
        storage_type: SliceBackedChoice,
    ) -> S
    where
        [(); check_empty_array_size(N)]:;
    fn collect_to_array<S: Slice<'a, Self::Item, N>, const N: usize>(self) -> S
    where
        [(); check_empty_array_size(N)]:;
    with_heap! {
        fn collect_to_vec<S: Slice<'a, Self::Item, N>, const N: usize>(self) -> S
        where
            [(); check_empty_array_size(N)]:;
    }
}
pub trait CollectToClone<'a>
where
    <Self as CollectToClone<'a>>::Item: 'a,
{
    type Item: Clone + PartialEq;
    fn collect_to_clone<S: SliceClone<'a, Self::Item, N>, const N: usize>(
        self,
        storage_type: SliceBackedChoice,
    ) -> S;
    fn collect_to_array_clone<S: SliceClone<'a, Self::Item, N>, const N: usize>(self) -> S;
    with_heap! {
        fn collect_to_vec_clone<S: SliceClone<'a, Self::Item, N>, const N: usize>(self) -> S;
    }
}

impl<'a, T: 'a + Clone + Copy + PartialEq, ITER: Iterator<Item = T>> CollectTo<'a> for ITER {
    type Item = T;
    fn collect_to<S: Slice<'a, Self::Item, N>, const N: usize>(
        self,
        storage_type: SliceBackedChoice,
    ) -> S
    where
        [(); check_empty_array_size(N)]:,
    {
        match_cfg! {storage_type,
            SliceBackedChoice::Array => self.collect_to_array(),
            ~[heap~]
            SliceBackedChoice::Vec => self.collect_to_vec(),
            _ => unimplemented!("Never")
        }
    }
    fn collect_to_array<S: Slice<'a, Self::Item, N>, const N: usize>(self) -> S
    where
        [(); check_empty_array_size(N)]:,
    {
        S::from_iter_to_array(self)
    }
    with_heap! {
        fn collect_to_vec<S: Slice<'a, Self::Item, N>, const N: usize>(self) -> S
        where
            [(); check_empty_array_size(N)]:,
        {
            S::from_iter_to_vec(self)
        }
    }
}

impl<'a, T: 'a + Clone + PartialEq, ITER: Iterator<Item = T>> CollectToClone<'a> for ITER {
    type Item = T;
    fn collect_to_clone<S: SliceClone<'a, Self::Item, N>, const N: usize>(
        self,
        storage_type: SliceBackedChoice,
    ) -> S {
        match_cfg! {storage_type,
            SliceBackedChoice::Array => self.collect_to_array_clone(),
            ~[heap~]
            SliceBackedChoice::Vec => self.collect_to_vec_clone(),
            _ => unimplemented!("Never")
        }
    }
    fn collect_to_array_clone<S: SliceClone<'a, Self::Item, N>, const N: usize>(self) -> S {
        S::from_iter_to_array(self)
    }
    with_heap! {
        fn collect_to_vec_clone<S: SliceClone<'a, Self::Item, N>, const N: usize>(self) -> S {
            S::from_iter_to_vec(self)
        }
    }
}

pub trait SliceDefault<'a, T: 'a + Clone + Copy + PartialEq + Default, const N: usize>
where
    Self: 'a,
{
    slice_trait!(SliceDefault);
    slice_trait_default!();
}

/// Slice/array/vector-based container, with extra abstractions. You can use it
/// on its own. The extra abstractions also make it compatible with (limited and
/// adapted) hash set/hash map. Slice or hash set/hash then serve as pluggable
/// in (range-based) `ranging::byte_slice::ByteSliceBoolStorage` and as
/// implementations of `ranging::set::Set` and `ranging::map::Map`.
///
/// If array-based, the size is fixed at compile time through a const generic
/// param `N`. If slice-based or vec-based, its size can be any, as given at
/// runtime. But for shared/mutable slice-based instances the size is fixed at
/// instantiation. Vec-based instances can be resized.
///
/// # Array size-related crate features
/// Param `N` indicates array size for SliceStorage::Array. It applies depending
/// on crate features `size_for_array_only`, `allow_empty_arrays` and
/// `disable_empty_arrays`.
///
/// # size_for_array_only
/// `size_for_array_only` in short: If `size_for_array_only` is enabled and size
/// `N` is non-zero, we allow SliceStorage::Array` variant only.
///
/// In detail:
///
/// If `N > 0`, that allows and reserves array storage in all variants of
///  `SliceStorage`. Then
///
/// -  if `size_for_array_only` is enabled, we allow `SliceStorage::Array`
/// variant only. And we forbid (at runtime) use of any other `SliceStorage`
/// variants (`SliceStorage::Shared`...) for non-zero `N`. (Even though this
/// check is specified for runtime, that is only due to Rust language
/// limitations. The check, if complied with, is likely to be optimized away at
/// compile time.)
///  
/// That prevents us from wasting memory (and possibly fragmenting CPU caches).
/// However, we have to type all non-array variants as having `N = 0`, and hence
/// we can't assign/pass those non-array variants to an array variant.
///
/// - if `size_for_array_only` is disabled, we allow any variants of
/// `SliceStorage` (as applicable to the choice of `std` or `no_std`). That
/// gives code flexibility (as we can assign/pass any variants), but each
/// `SliceStorage` takes that array space - even if not used.
///
/// If `N = 0`, we either allow an array of size `0`, or we disable array
/// variant.
///
/// - if `size_for_array_only` is enabled, we forbid (runtime) use of
///  `SliceStorage::Array` (with that `N = 0`). Use slice/vector-based variants
/// of `SliceStorage` (`SliceStorage::Shared`..., `SliceStorage::Vec`...)
///  instead.
///
/// - if `size_for_array_only` is disabled, we allow array storage with size `0`
/// (for `SliceStorage::Array`) and we allow all other variants (as applicable
/// according to `std` or `no_std`), too.
///
/// # Usage in library crates & size_for_array_only
/// ## Make it flexible
/// - use non-zero `N` only for array-based purposes (often on stack, or in
///   `no_std`). Otherwise use zero `N` for code pathways based on a
///   slice/vector.
/// - if you see frequent code pathways that copy/clone `Slice`s, and if you
///   have heap, consider `to_non_array_vec_based()`.
/// - check with `is_owned()` and have two pathways, if simple enough
/// - use `to_*()` methods.
///
/// Don't hard code any non-zero `N` (unless sure), but have it come from the
/// client.
///
/// ## Test
/// Ship your crate without enabling `size_for_array_only`, but test it both
/// with and without `size_for_array_only`. For that you need tests in separate
/// test crates, so that you isolate this feature.
///
/// ## Test with size_for_array_only
/// Enable `size_for_array_only` and test with  both zero `N` size and non-zero
/// `N`. Only some methods/invariants will work for one, and some for the other.
///
/// ## Test without size_for_array_only
/// Don't enable `size_for_array_only` and test with  both zero `N` size and
/// non-zero `N`. Again, only some methods/invariants will work, respectively.
///
/// # allow_empty_arrays and disable_empty_array
/// In short: Disabling empty arrays (invariants) is possible, but only
/// globally. It applies to any implementations of Slice* traits and
/// SliceStorage* and related structs/enums.
///
/// `Slice` (and it implementations) have `const N: usize` and not `const N:
/// Option<usize>`. The later (or a similar custom enum) would allow granular
/// (per generic type instance) intent as to whether the specific type (for its
/// chosen `N`) allows arrays of the given size (whether zero or not), or
/// whether it disables its array-based variant completely.
///
/// However, that would require extra bounds like `where [(); N.unwrap_or(0)]:`,
/// not only in the `trait`s and `struct`s & `impl`, but also in any client
/// code! Very unergonomic.
///
/// Hence we have `const N: usize` instead. That means that specific types can't
/// control whether they completely disable any array invariants, or whether
/// they enable empty arrays. Such difference is possible only globally with
/// crate features `allow_empty_arrays` & `disable_empty_arrays`. Those two
/// features are mutually exclusive. If none of them is set, empty arrays are
/// allowed by default.
///
/// # Naming convention for methods:
/// - `as_***()` means conversion (sharing), but not a copy - TODO reconsider &
///   implement
/// - `to_***()` means a copy (NOT currently; TODO reconsider).

/// If `T` is `Copy`, use `Slice` instead of `SliceClone`. See also
/// `SliceStorageClone` and `SliceStorage` for reasoning on why they are named
/// so.
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

enum_cfg! {
    (
        /// Indicate the storage choice. There is no "BoxedArray" invariant,
        /// because having a `Box` of an array would mean either wasting the
        /// unboxed array invariant's space, or introducing another const
        /// generic parameter (which would make it much less ergonomic).
        #[derive(Debug, PartialEq)]
        pub enum SliceBackedChoice
    ),
    Shared,
    Mutable,
    Array,
    ~[heap~]
    Vec,
    ~[heap~]
    VecRef
}

impl SliceBackedChoice {
    pub fn is_owned(&self) -> bool {
        use SliceBackedChoice::*;
        match_cfg! { self,
            Shared | Mutable => false,
            Array => true,
            ~[heap~]
            VecRef => false,
            ~[heap~]
            Vec => true
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array)
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
        enum_cfg! {
            (
                /// See also [SliceStorage and
                /// similar](../index.html#slicestorage-and-similar).
                #[derive(Debug)]
                pub enum $enum_name<'a, T: 'a + $($item_bounds)+, const N: usize>
            ),
            Shared(&'a [T]),
            Mutable(&'a mut [T]),

            /// Owned array. Suggested for stack & `no_std`.
            Array([T; N]),

            /// Owned vector.
            ~[heap~]
            Vec(Vec<T>),

            ~[heap~]
            VecRef(&'a mut Vec<T>)
        }
    }
}

slice_storage_enum!(SliceStorage, Clone + Copy);
slice_storage_enum!(SliceStorageClone, Clone);
slice_storage_enum!(SliceStorageDefault, Clone + Copy + Default);
slice_storage_enum!(SliceStorageDefaultClone, Clone + Default);

fn copy_value<T: Clone + Copy>(from: &T) -> T {
    *from
}
fn clone_value<T: Clone>(from: &T) -> T {
    from.clone()
}
fn copy_to_array<T: Clone + Copy, const N: usize>(from: &T) -> [T; N] {
    [*from; N]
}
fn clone_to_array<T: Clone, const N: usize>(from: &T) -> [T; N] {
    core::array::from_fn(|_| from.clone())
}
fn fn_to_array<T: Clone, const N: usize>(mut f: impl FnMut() -> T) -> [T; N] {
    core::array::from_fn(|_| f())
}

macro_rules! slice_storage_impl {
    ($enum_name:ident, $copy_or_clone_value: ident, $copy_or_clone_to_array: ident) => {
        type ITER<'i> = core::slice::Iter<'i, T>
            where T: 'i, Self: 'i;

        with_heap!{
            type NARR = $enum_name<'a, T, 0>;
        }

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
            // Since N is const, this assert may be optimized away.
            #[cfg(feature = "size_for_array_only")]
            assert_eq!(N, 0);
            Self::Shared(slice)
        }
        fn from_mutable(slice: &'a mut [T]) -> Self {
            // Since N is const, this assert may be optimized away.
            #[cfg(feature = "size_for_array_only")]
            assert_eq!(N, 0);
            Self::Mutable(slice)
        }
        fn from_array(array: [T; N]) -> Self {
            // debug (non-optimized build)-only check is enough, because we also
            // have a compile-time check by bounds.
            #[cfg(feature = "disable_empty_arrays")]
            debug_assert!(N > 0);
            Self::Array(array)
        }

        with_heap! {
            fn from_vec(vector: Vec<T>) -> Self {
                // Since N is const, this assert may be optimized away.
                #[cfg(feature = "size_for_array_only")]
                assert_eq!(N, 0);
                Self::Vec(vector)
            }
        }
        with_heap! {
            fn from_vec_ref(vector: &'a mut Vec<T>) -> Self {
                Self::VecRef(vector)
            }
        }
        fn from_value_to_array(value_ref: &'a T) -> Self {
            Self::Array($copy_or_clone_to_array(value_ref))
        }

        with_heap! {
            fn from_value_to_vec(value: &'a T, size: usize) -> Self {
                let mut vec = Vec::with_capacity(size);
                for _ in 0..size {
                    vec.push($copy_or_clone_value(value));
                }
                Self::Vec(vec)
            }
        }
        fn from_iter_to_array(mut iter: impl Iterator<Item = T>) -> Self {
            Self::Array(fn_to_array(|| iter.next().unwrap()))
        }
        with_heap! {
            fn from_iter_to_vec(iter: impl Iterator<Item = T>) -> Self {
                Self::Vec(iter.collect::<Vec<_>>())
            }
        }

        fn from_fn_to_array(f: impl FnMut() -> T) -> Self {
            Self::Array(fn_to_array(f))
        }
        with_heap! {
            fn from_fn_to_vec(mut f: impl FnMut() -> T, size: usize) -> Self {
                Self::Vec((0..size).map(|_| f()).collect::<Vec<_>>())
            }
        }
        with_heap! {
            /// Return `self` if `Vec`-based, otherwise a new `Vec`-based
            /// instance populated from `self`.
            fn to_vec_based(self) -> Self {
                match self {
                    // @TODO Vec::with_capacity <- slice.len: --\\
                    Self::Shared(slice) => Self::Vec(slice.iter().cloned().collect::<Vec<_>>()),
                    // @TODO Vec::with_capacity <- slice.len: --\\
                    Self::Mutable(mutable) => Self::Vec(mutable.iter().cloned().collect::<Vec<_>>()),
                    // @TODO Vec::with_capacity --\\
                    Self::Array(arr) => Self::Vec(arr.iter().cloned().collect::<Vec<_>>()),
                    Self::Vec(_) => self,
                    Self::VecRef(_) => self,
                }
            }
        }
        with_heap! {
            fn to_non_array_vec_based(&self) -> Self::NARR {
                // @TODO with_capacity
                let v: Vec<T>;
                if let Self::Mutable(mutable_slice) = self {
                    v = Vec::from_iter(mutable_slice.iter().cloned());
                } else {
                    let slice = match self {
                        Self::Array(array) => array,
                        Self::Shared(shared_slice) => *shared_slice,
                        Self::Vec(vec) => vec,
                        Self::VecRef(vec_ref) => *vec_ref,
                        Self::Mutable(_) => unreachable!(),
                    };
                    v = Vec::from_iter(slice.iter().cloned());
                }
                Self::NARR::Vec(v)
            }
        }

        with_heap! {
            fn from_non_array_vec_based(from: &Self::NARR,
                as_choice: &SliceBackedChoice
            ) -> Self {
                use SliceBackedChoice::*;
                let from_vec;
                match from {
                    Self::NARR::Vec(vec) => from_vec = vec,
                    _ => unreachable!("NARR must always be Vec-based!")
                }
                match as_choice {
                    Shared | Mutable => unimplemented!("Never"),
                    Array => {
                        Self::Array(array::from_fn(|i| from_vec[i].clone()))
                    },
                    Vec => {
                        Self::from_vec(from_vec.clone())
                    },
                    VecRef => unimplemented!("Never"),
                }
            }
        }

        // Accessors
        fn shared_slice(&self) -> &[T] {
            match_cfg! {&self,
                Self::Shared(slice) => slice,
                Self::Mutable(slice) => slice,
                Self::Array(array) => array,
                ~[heap~]
                Self::Vec(vec) => vec,
                ~[heap~]
                Self::VecRef(vec_ref) => *vec_ref
            }
        }

        fn mutable_slice<'s>(&'s mut self) -> &'s mut [T] {
            match_cfg! {self,
                Self::Shared(_) => {
                    unimplemented!("Can't get a mutable slice from a shared slice.")
                },
                Self::Mutable(slice) => slice,
                Self::Array(array) => array,
                ~[heap~]
                Self::Vec(vec) => vec,
                ~[heap~]
                Self::VecRef(vec_ref) => *vec_ref
            }
        }

        with_heap! {
            fn mutable_vec<'s>(&'s mut self) -> &'s mut Vec<T> {
                match self {
                    Self::Vec(vec) => vec,
                    Self::VecRef(vec_ref) => *vec_ref,
                    _ => {
                        unimplemented!("Works for Vec and VecRef only.")
                    }
                }
            }
        }
    };
}

impl<'a, T: 'a + Clone + Copy + PartialEq, const N: usize> Slice<'a, T, N>
    for SliceStorage<'a, T, N>
where
    [(); check_empty_array_size(N)]:,
{
    slice_storage_impl!(SliceStorage, copy_value, copy_to_array);
}
impl<'a, T: 'a + Clone + PartialEq, const N: usize> SliceClone<'a, T, N>
    for SliceStorageClone<'a, T, N>
{
    slice_storage_impl!(SliceStorageClone, clone_value, clone_to_array);
}

// Following functions are in pairs, used as alternative implementation parts
// passed to slice_storage_default_impl! macro.
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
// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=efa6fb4de4d30d4813a8790fefbb7bed#ARRAY
fn clone_array_default<T: Clone + Default, const N: usize>() -> [T; N] {
    core::array::from_fn(|_| T::default())
}
macro_rules! slice_storage_default_impl {
    ($copy_or_clone_from_slice: ident, $copy_or_clone_array: ident, $copy_or_clone_default: ident) => {
        fn to_array_based(&self) -> Self {
            // debug (non-optimized build)-only check is enough, because we also
            // have a compile-time check by bounds.
            #[cfg(feature = "disable_empty_arrays")]
            debug_assert!(N > 0);

            match_cfg! {self,
                Self::Array(from) => {
                    let to = $copy_or_clone_array(from);
                    Self::Array(to)
                },
                Self::Shared(slice) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(*slice);
                    Self::Array(to)
                },
                Self::Mutable(slice) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(*slice);
                    Self::Array(to)
                },
                ~[heap~]
                Self::Vec(vec) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(vec);
                    Self::Array(to)
                },
                ~[heap~]
                Self::VecRef(vec_ref) => {
                    let mut to = $copy_or_clone_default();
                    to.$copy_or_clone_from_slice(*vec_ref);
                    Self::Array(to)
                }
            }
        }

        fn from_default_to_array() -> Self {
            Self::Array($copy_or_clone_default())
        }
        with_heap! {
            fn from_default_to_vec(size: usize) -> Self {
                let mut vec = Vec::with_capacity(size);
                for _ in 0..size {
                    vec.push(T::default());
                }
                Self::Vec(vec)
            }
        }
    };
}

impl<'a, T: 'a + Clone + Copy + PartialEq + Default, const N: usize> SliceDefault<'a, T, N>
    for SliceStorageDefault<'a, T, N>
{
    slice_storage_impl!(SliceStorageDefault, copy_value, copy_to_array);
    slice_storage_default_impl!(copy_from_slice, copy_array, copy_array_default);
}
impl<'a, T: 'a + Clone + PartialEq + Default, const N: usize> SliceDefaultClone<'a, T, N>
    for SliceStorageDefaultClone<'a, T, N>
{
    slice_storage_impl!(SliceStorageDefaultClone, clone_value, clone_to_array);
    slice_storage_default_impl!(clone_from_slice, clone_array, clone_array_default);
}

macro_rules! slice_storage_impl_clone {
    () => {
        /// Implemented for Array-backed and Vec-backed SliceStorage only. For
        /// Vec (mutable) reference-backed SliceStorage this creates a new,
        /// owned Vec-based instance.
        fn clone(&self) -> Self {
            match_cfg! {self,
                Self::Shared(_) => {
                    unimplemented!("Don't clone a shared slice-backed SliceStorage. Instead, pass a shared reference to it.")
                },
                Self::Mutable(_) => {
                    unimplemented!("Can't clone a mutable slice.")
                },
                Self::Array(array) => Self::Array(array.clone()),
                ~[heap~]
                Self::Vec(vec) => Self::Vec(vec.clone()),
                // Can't clone a mutable reference. Clone the vector itself.
                ~[heap~]
                Self::VecRef(vec_ref) => Self::Vec((*vec_ref).clone())
            }
        }
    }
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

macro_rules! slice_storage_newlike_impl {
    ($copy_or_clone_default: ident) => {
        /// Implemented for Shared-backed, Array-backed and Vec-backed (but not
        /// VecRef-backed) variants only.
        fn new_empty_like(&self) -> Self {
            match_cfg! {self,
                Self::Shared(slice) => Self::Shared(slice),
                Self::Mutable(_) => {
                    unimplemented!("Can't clone a mutable slice.")
                },
                Self::Array(_) => Self::Array($copy_or_clone_default()),
                ~[heap~]
                Self::Vec(vec) => Self::Vec(Vec::with_capacity(vec.len())),
                ~[heap~]
                Self::VecRef(_) => {
                    unimplemented!("Can't clone a mutable Vec reference.")
                }
            }
        }
    };
}

impl<'s, T: 's + Clone + Copy + Default, const N: usize> crate::abstra::NewEmptyLike
    for SliceStorageDefault<'s, T, N>
{
    slice_storage_newlike_impl!(copy_array_default);
}
impl<'s, T: 's + Clone + Default, const N: usize> crate::abstra::NewEmptyLike
    for SliceStorageDefaultClone<'s, T, N>
{
    slice_storage_newlike_impl!(clone_array_default);
}

pub type BoolSlice<'a, const N: usize> = SliceStorageDefault<'a, bool, N>;
pub type ByteSlice<'a, const N: usize> = SliceStorageDefault<'a, u8, N>;
