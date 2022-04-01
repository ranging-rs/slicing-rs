/// Const generic param `N` is used by `Slice::Array` only. (However, it makes all variants consume space. Hence:) Suggested for `no_std` only.
/// If you run in `std`, suggest passing 0 for `N`, and use `Slice::Vec` instead.
#[derive(Debug)]
pub enum Slice<'a, T: 'a, const N: usize> {
    Shared(&'a [T]),
    Mutable(&'a mut [T]),
    /// Owned array. Suggested for no_std only.
    Array([T; N]),
    #[cfg(all(not(feature = "no_std"), feature = "std"))]
    /// Owned vector. For std only.
    Vec(Vec<T>),
}

impl<'s, T: 's, const N: usize> Slice<'s, T, N> {
    #[inline]
    pub fn shared_slice<'a>(&'a self) -> &'a [T] {
        match &self {
            Slice::Shared(slice) => slice,
            Slice::Mutable(slice) => slice,
            Slice::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            Slice::Vec(vec) => vec,
        }
    }

    #[inline]
    /// Implemented for all except for Shared-based slice.
    pub fn mutable_slice<'a>(&'a mut self) -> &'a mut [T] {
        match self {
            Slice::Shared(_) => {
                unimplemented!("Can't get a mutable slice from a shared slice.")
            }
            Slice::Mutable(slice) => slice,
            Slice::Array(array) => array,
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            Slice::Vec(vec) => vec,
        }
    }
}

impl<'s, T: 's + Clone, const N: usize> Clone for Slice<'s, T, N> {
    /// Implemented for Array-backed and Vec-backed slice only.
    fn clone(&self) -> Self {
        match self {
            Slice::Shared(_) | Slice::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            Slice::Array(array) => Slice::Array(array.clone()),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            Slice::Vec(vec) => Slice::Vec(vec.clone()),
        }
    }
}

impl<'s, T: 's + Clone + Copy + Default, const N: usize> crate::abstra::NewLike
    for Slice<'s, T, N>
{
    /// Implemented for Array-backed and Vec-backed slice only.
    fn new_like(&self) -> Self {
        match self {
            Slice::Shared(_) | Slice::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            Slice::Array(_) => Slice::Array([T::default(); N]),
            #[cfg(all(not(feature = "no_std"), feature = "std"))]
            Slice::Vec(_) => Slice::Vec(if N > 0 {
                Vec::with_capacity(N)
            } else {
                Vec::new()
            }),
        }
    }
}

pub type BoolSlice<'a, const N: usize> = Slice<'a, bool, N>;
pub type ByteSlice<'a, const N: usize> = Slice<'a, u8, N>;
