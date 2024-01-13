use std::collections::HashSet;

use crate::InserttableContainer;
use crate::RemovableContainer;


impl<T> InserttableContainer<T> for Vec<T> {
    fn insert(&mut self, item: T) {
        self.push(item);
    }
}

impl<T> RemovableContainer<T> for Vec<T> {
    fn remove(&mut self) -> Option<T> {
        self.pop()
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
    
}

impl<T> InserttableContainer<T> for HashSet<T> 
where T: Eq + std::hash::Hash
{
    fn insert(&mut self, item: T) {
        HashSet::insert(self, item);
    }
}

impl<T> RemovableContainer<T> for HashSet<T> 
where T: Eq + std::hash::Hash + Clone
{
    // Remove an arbitrary item from the set and return it
    fn remove(&mut self) -> Option<T> {
        let item = self.iter().next().cloned();
        if let Some(ref item) = item {
            HashSet::remove(self, item);
        }
        item
    }

    fn len(&self) -> usize {
        HashSet::len(self)
    }
}