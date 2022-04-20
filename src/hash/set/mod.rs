use core::hash::Hash;
use std::collections::{hash_set, HashSet};

#[cfg(not(feature = "no_std"))]
#[derive(Debug)]
pub struct HashedSet<T> {
    set: HashSet<T>,
}

#[cfg(not(feature = "no_std"))]
impl<T: Hash + Eq + Clone> crate::set::Set<T> for HashedSet<T> {
    type ITER<'a> = HashedSetIter<'a, T>
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
}
#[cfg(not(feature = "no_std"))]
impl<T: Hash + Eq + Clone> crate::abstra::NewLike for HashedSet<T> {
    fn new_like(&self) -> Self {
        Self {
            set: HashSet::<T>::new(),
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl<T: Hash + Eq + Clone> Clone for HashedSet<T> {
    fn clone(&self) -> Self {
        Self {
            set: self.set.clone(),
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
        //self.set_iter.next().map(|value| value.clone())
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
