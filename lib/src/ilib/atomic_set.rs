use std::ops::Deref;
use std::fmt::{Debug, self};
use std::sync::atomic::{ AtomicPtr, Ordering };
use std::ptr::null_mut;

// once-settable, any-readonly-gettable option
// useful for read-only structs with after-initialization, one-time fill-in of data
pub struct AtomicSet<T: Send + Sync + Sized> {
    item: AtomicPtr<T>,
}

impl<T: Send + Sync + Sized> Default for AtomicSet<T> {
    fn default() -> AtomicSet<T> {
        AtomicSet::new()
    }
}

impl<T: Send + Sync + Sized> Deref for AtomicSet<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}

impl<T: Debug + Send + Sync + Sized> Debug for AtomicSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Clone + Send + Sync + Sized> Clone for AtomicSet<T> {
    fn clone(&self) -> Self {
        let ptr = self.item.load(Ordering::Relaxed);
        let ptr = if ptr == null_mut::<T>() {
            ptr
        } else {
            Box::into_raw(Box::new(unsafe { ptr.as_ref() }.unwrap().clone()))
        };
        AtomicSet {
            item: AtomicPtr::new(ptr),
        }
    }
}

impl<T: PartialEq + Send + Sync + Sized> PartialEq for AtomicSet<T> {
    fn eq(&self, other: &Self) -> bool {
        return **self == **other;
    }
}

impl<T: Send + Sync + Sized> Drop for AtomicSet<T> {
    fn drop(&mut self) {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr != null_mut::<T>() {
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

impl<T: Send + Sync + Sized> AtomicSet<T> {
    
    pub fn new() -> AtomicSet<T> {
        AtomicSet {
            item: AtomicPtr::new(null_mut::<T>()),
        }
    }

    pub fn try_set(&self, item: T) {
        let boxed_item = Box::into_raw(Box::new(item));
        let null_ptr = null_mut::<T>();
        if self.item.compare_and_swap(null_ptr, boxed_item, Ordering::Relaxed) != null_ptr {
            drop(unsafe { Box::from_raw(boxed_item) });
        }
    }

    pub fn set(&self, item: T) {
        let boxed_item = Box::into_raw(Box::new(item));
        let null_ptr = null_mut::<T>();
        if self.item.compare_and_swap(null_ptr, boxed_item, Ordering::Relaxed) != null_ptr {
            drop(unsafe { Box::from_raw(boxed_item) });
            panic!("attempted to re-set AtomicSet!");
        }
    }

    pub fn get(&self) -> &T {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr == null_mut::<T>() {
            panic!("attempt to get value of unset AtomicSet!");
        }
        unsafe { ptr.as_ref() }.unwrap()
    }

    pub fn is_set(&self) -> bool {
        let ptr = self.item.load(Ordering::Relaxed);
        ptr != null_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_set() {
        let setter = AtomicSet::<u32>::new();
        assert_eq!(setter.is_set(), false);
        setter.set(12345);
        assert_eq!(setter.is_set(), true);

        assert_eq!(*setter.get(), 12345);
        assert_eq!(*setter.get(), 12345);
        assert_eq!(*setter, 12345);
        assert_eq!(setter.is_set(), true);
    }

    #[test]
    #[should_panic]
    fn test_cannot_set_twice() {
        let setter = AtomicSet::<u32>::new();
        setter.set(12345);
        setter.set(12345);
    }
}