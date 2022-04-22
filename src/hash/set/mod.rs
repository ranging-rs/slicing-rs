use crate::abstra::NewLike;
use crate::set::Set;
use core::hash::Hash;
use std::collections::{hash_set, hash_set::Iter, HashSet};

#[cfg(not(feature = "no_std"))]
#[derive(Debug, Clone)]
pub struct HashedSet<T> {
    set: HashSet<T>,
}

#[cfg(not(feature = "no_std"))]
impl<T: Hash + Eq + Clone> Set<T> for HashedSet<T> {
    type ITER<'a> = HashedSetIter<'a, T>
    where
        T: 'a,
        Self: 'a,
    ;

    type ITERREF<'a> = Iter<'a, T>
    where
        T: 'a,
        Self: 'a,
    ;

    fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
    fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }
    fn remove(&mut self, value: &T) -> bool {
        self.set.remove(value)
    }
    fn iter<'a>(&'a self) -> Self::ITER<'a> {
        HashedSetIter {
            set_iter: self.set.iter(),
        }
    }

    fn supports_iter_ref() -> bool {
        true
    }

    fn iter_ref<'a>(&'a self) -> Self::ITERREF<'a> {
        self.set.iter()
    }
}
#[cfg(not(feature = "no_std"))]
impl<T: Hash + Eq + Clone> NewLike for HashedSet<T> {
    fn new_like(&self) -> Self {
        Self {
            set: HashSet::<T>::new(),
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl<T: Hash + Eq> HashedSet<T> {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
    pub fn new_with(set: HashSet<T>) -> Self {
        Self { set }
    }
}

#[cfg(not(feature = "no_std"))]
pub struct HashedSetIter<'a, T: 'a> {
    set_iter: hash_set::Iter<'a, T>,
}

#[cfg(not(feature = "no_std"))]
impl<'a, T: Clone> Iterator for HashedSetIter<'a, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        self.set_iter.next().cloned()
    }
}

#[cfg(not(feature = "no_std"))]
impl<T: core::hash::Hash + Eq> FromIterator<T> for HashedSet<T> {
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = T>,
    {
        Self {
            set: HashSet::from_iter(iter),
        }
    }
}
