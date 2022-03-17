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
                unreachable!("Can't get a mutable slice from a shared slice.")
            }
            Slice::Mutable(slice) => slice,
            #[cfg(not(feature = "no_std"))]
            Slice::Vec(vec) => vec,
        }
    }
}

pub type BoolSlice<'a> = Slice<'a, bool>;
pub type ByteSlice<'a> = Slice<'a, u8>;
