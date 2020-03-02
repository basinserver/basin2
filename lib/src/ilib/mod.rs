mod atomic_set;
mod whitelist;
mod registry;
pub use atomic_set::AtomicSet;
pub use whitelist::Whitelist;
pub use registry::{ Registry, RegistryItem };
use std::iter::FromIterator;
use crate::Result;

pub trait TryCollect<K>: Iterator {
    fn try_collect<B: FromIterator<K>>(self) -> Result<B> where Self: Sized;
}

impl<K, T: Iterator<Item=Result<K>>> TryCollect<K> for T {
    fn try_collect<B: FromIterator<K>>(self) -> Result<B> where Self: Sized {
        let collected: Vec<Result<K>> = self.collect();
        let mut results = vec![];
        for item in collected {
            results.push(item?);
        }
        return Ok(B::from_iter(results));
    }
}