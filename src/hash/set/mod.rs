use core::hash::Hash;
use std::collections::{hash_set, HashSet};

#[derive(Debug)]
pub struct HashedSet<T> {
    set: HashSet<T>,
}

impl<T: Hash + Eq + Clone> crate::Set<T> for HashedSet<T> {
    type ITER<'a>
    where
        T: 'a,
        Self: 'a,
    = HashedSetIter<'a, T>;
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

    fn new_like(&self) -> Self {
        Self {
            set: HashSet::<T>::new(),
        }
    }
}

impl<T: Hash + Eq + Clone> Clone for HashedSet<T> {
    fn clone(&self) -> Self {
        Self {
            set: self.set.clone(),
        }
    }
}

impl<T: Hash + Eq> HashedSet<T> {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
}

pub struct HashedSetIter<'a, T: 'a> {
    set_iter: hash_set::Iter<'a, T>,
}
impl<'a, T: Clone> Iterator for HashedSetIter<'a, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        self.set_iter.next().map(|value| value.clone())
    }
}

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
