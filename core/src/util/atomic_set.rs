use std::cell::UnsafeCell;
use std::sync::Mutex;
use std::ops::Deref;

// once-settable, any-readonly-gettable option
// useful for read-only structs with after-initialization, one-time fill-in of data
pub struct AtomicSet<T: Send + Sync + Sized> {
    item: UnsafeCell<Option<T>>,
    mutex: Mutex<()>,
}

unsafe impl<T: Send + Sync + Sized> Send for AtomicSet<T> {
    
}

unsafe impl<T: Send + Sync + Sized> Sync for AtomicSet<T> {
    
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

impl<T: Send + Sync + Sized> AtomicSet<T> {
    
    pub fn new() -> AtomicSet<T> {
        AtomicSet {
            item: UnsafeCell::new(None),
            mutex: Mutex::new(()),
        }
    }

    pub fn try_set(&self, item: T) {
        let _ = self.mutex.lock().unwrap();
        unsafe {
            if self.item.get().as_ref().unwrap().is_some() {
                return;
            }
            self.item.get().as_mut().unwrap().replace(item);
        }
    }

    pub fn set(&self, item: T) {
        let _ = self.mutex.lock().unwrap();
        unsafe {
            if self.item.get().as_ref().unwrap().is_some() {
                panic!("attempted to re-set AtomicSet!");
            }
            self.item.get().as_mut().unwrap().replace(item);
        }
    }

    pub fn get(&self) -> &T {
        unsafe {
            self.item.get().as_ref().unwrap().as_ref().unwrap()
        }
    }

    pub fn is_set(&self) -> bool {
        unsafe {
            self.item.get().as_ref().unwrap().is_some()
        }
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