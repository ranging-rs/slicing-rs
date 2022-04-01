#[derive(Debug)]
pub enum Slice<'a, T: 'a> {
    Shared(&'a [T]),
    Mutable(&'a mut [T]),
    #[cfg(not(feature = "no_std"))]
    Vec(Vec<T>),
}

impl<'s, T: 's> Slice<'s, T> {
    #[inline]
    pub fn shared_slice<'a>(&'a self) -> &'a [T] {
        match &self {
            Slice::Shared(slice) => slice,
            Slice::Mutable(slice) => slice,
            #[cfg(not(feature = "no_std"))]
            Slice::Vec(vec) => vec,
        }
    }

    #[inline]
    pub fn mutable_slice<'a>(&'a mut self) -> &'a mut [T] {
        match self {
            Slice::Shared(_) => {
                unimplemented!("Can't get a mutable slice from a shared slice.")
            }
            Slice::Mutable(slice) => slice,
            #[cfg(not(feature = "no_std"))]
            Slice::Vec(vec) => vec,
        }
    }
}

/// Implemented for Vec-backed slice only.
impl<'s, T: 's + Clone> Clone for Slice<'s, T> {
    fn clone(&self) -> Self {
        match self {
            Slice::Shared(_) | Slice::Mutable(_) => {
                unimplemented!("Can't clone a slice.")
            }
            #[cfg(not(feature = "no_std"))]
            Slice::Vec(vec) => Slice::Vec(vec.clone()),
        }
    }
}

pub type BoolSlice<'a> = Slice<'a, bool>;
pub type ByteSlice<'a> = Slice<'a, u8>;
