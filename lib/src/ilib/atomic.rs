use std::fmt::{Debug, self};
use std::sync::atomic::{ AtomicPtr, Ordering };
use std::ptr::null_mut;

// once-settable, any-readonly-gettable option
// useful for read-only structs with after-initialization, one-time fill-in of data
pub struct Atomic<T: Send + Sync + Clone + Copy + 'static> {
    item: AtomicPtr<T>,
}

impl<T: Send + Sync + Clone + Copy + 'static> Default for Atomic<T> {
    fn default() -> Atomic<T> {
        Atomic::new()
    }
}

impl<T: Debug + Send + Sync + Clone + Copy + 'static> Debug for Atomic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: Send + Sync + Clone + Copy + 'static> Clone for Atomic<T> {
    fn clone(&self) -> Self {
        let ptr = self.item.load(Ordering::Relaxed);
        let ptr = if ptr == null_mut::<T>() {
            ptr
        } else {
            Box::into_raw(Box::new(self.get())) as *mut T
        };
        Atomic {
            item: AtomicPtr::new(ptr),
        }
    }
}

impl<T: PartialEq + Send + Sync + Clone + Copy + 'static> PartialEq for Atomic<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.get() == other.get();
    }
}

impl<T: Send + Sync + Clone + Copy + 'static> Drop for Atomic<T> {
    fn drop(&mut self) {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr != null_mut() {
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

impl<T: Send + Sync + Clone + Copy + 'static> From<T> for Atomic<T> {
    fn from(item: T) -> Atomic<T> {
        return Atomic {
            item: AtomicPtr::new(Box::into_raw(Box::new(item)) as *mut T),
        };
    }
}

impl<T: Send + Sync + Clone + Copy + 'static> Atomic<T> {
    
    pub fn new() -> Atomic<T> {
        Atomic {
            item: AtomicPtr::new(null_mut::<T>()),
        }
    }

    pub fn set(&self, item: T) {
        let boxed_item = Box::into_raw(Box::new(item)) as *mut T;
        let old_item = self.item.swap(boxed_item, Ordering::Relaxed);
        if old_item != null_mut() {
            drop(unsafe { Box::from_raw(old_item) });
        }
    }

    pub fn get(&self) -> T {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr == null_mut::<T>() {
            panic!("attempt to get value of unset Atomic!");
        }
        return unsafe { *ptr };
    }

    pub fn is_set(&self) -> bool {
        let ptr = self.item.load(Ordering::Relaxed);
        ptr != null_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct TestStruct {
        value: u32,
    }

    #[test]
    fn test_ref_can_set() {
        let setter = Atomic::<TestStruct>::new();
        assert_eq!(setter.is_set(), false);
        setter.set(TestStruct { value: 243523 });
        assert_eq!(setter.is_set(), true);

        assert_eq!(setter.get(), TestStruct { value: 243523 });
        assert_eq!(setter.is_set(), true);
        let gotten = setter.get();
        drop(setter);
        assert_eq!(gotten, TestStruct { value: 243523 });
    }

}