use std::iter::Iterator;
use std::hash::Hash;
use std::collections::HashSet;
use std::iter::FromIterator;

pub trait Whitelist<T: Eq> {
    fn whitelist(&mut self, valid: &Vec<T>) -> bool;
}

impl<X: Eq + Hash, T: Iterator<Item=X>> Whitelist<X> for T {
    fn whitelist(&mut self, valid: &Vec<X>) -> bool {
        let set: HashSet<&X> = HashSet::from_iter(valid);
        for item in self {
            if !set.contains(&item) {
                return false;
            }
        }
        true
    }
}
