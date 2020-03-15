use std::fmt::{Debug, self};
use std::sync::atomic::{ AtomicPtr, Ordering };
use std::sync::Arc;
use std::ptr::null_mut;
use std::ops::Deref;

// n-settable, any-readonly-gettable option
pub struct AtomicRef<T: Send + Sync + ?Sized + 'static> {
    item: AtomicPtr<Arc<T>>,
}

impl<T: Send + Sync + ?Sized + 'static> Default for AtomicRef<T> {
    fn default() -> AtomicRef<T> {
        AtomicRef::new()
    }
}

impl<T: Debug + Send + Sync + ?Sized + 'static> Debug for AtomicRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self.get()).fmt(f)
    }
}

impl<T: Clone + Send + Sync + ?Sized + 'static> Clone for AtomicRef<T> {
    fn clone(&self) -> Self {
        let ptr = self.item.load(Ordering::Relaxed);
        let ptr = if ptr == null_mut::<Arc<T>>() {
            ptr
        } else {
            Box::into_raw(Box::new(self.get())) as *mut Arc<T>
        };
        AtomicRef {
            item: AtomicPtr::new(ptr),
        }
    }
}

impl<T: PartialEq + Send + Sync + ?Sized + 'static> PartialEq for AtomicRef<T> {
    fn eq(&self, other: &Self) -> bool {
        return (*self.get()) == (*other.get());
    }
}

impl<T: Send + Sync + ?Sized + 'static> Drop for AtomicRef<T> {
    fn drop(&mut self) {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr != null_mut() {
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

impl<T: Send + Sync + ?Sized + 'static> From<Arc<T>> for AtomicRef<T> {
    fn from(item: Arc<T>) -> AtomicRef<T> {
        return AtomicRef {
            item: AtomicPtr::new(Box::into_raw(Box::new(item)) as *mut Arc<T>),
        };
    }
}

impl<T: Send + Sync + ?Sized + 'static> Deref for AtomicRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr == null_mut::<Arc<T>>() {
            panic!("attempt to get value of unset AtomicRef!");
        }
        unsafe { ptr.as_ref().unwrap() }
    }
}

impl<T: Send + Sync + ?Sized + 'static> AtomicRef<T> {
    
    pub fn new() -> AtomicRef<T> {
        AtomicRef {
            item: AtomicPtr::new(null_mut::<Arc<T>>()),
        }
    }

    pub fn set(&self, item: Arc<T>) {
        let boxed_item = Box::into_raw(Box::new(item)) as *mut Arc<T>;
        let old_item = self.item.swap(boxed_item, Ordering::Relaxed);
        if old_item != null_mut() {
            drop(unsafe { Box::from_raw(old_item) });
        }
    }

    pub fn get(&self) -> Arc<T> {
        let ptr = self.item.load(Ordering::Relaxed);
        if ptr == null_mut::<Arc<T>>() {
            panic!("attempt to get value of unset AtomicRef!");
        }
        let original_arc = unsafe { Box::from_raw(ptr) };
        let returning_arc = (*original_arc).clone();
        Box::into_raw(original_arc);
        return returning_arc;
    }

    pub fn is_set(&self) -> bool {
        let ptr = self.item.load(Ordering::Relaxed);
        ptr != null_mut::<Arc<T>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestStruct {
        value: u32,
    }

    #[test]
    fn test_ref_can_set() {
        let setter = AtomicRef::<TestStruct>::new();
        assert_eq!(setter.is_set(), false);
        setter.set(Arc::new(TestStruct { value: 243523 }));
        assert_eq!(setter.is_set(), true);

        assert_eq!(*setter.get(), TestStruct { value: 243523 });
        assert_eq!(setter.is_set(), true);
        let gotten = setter.get();
        drop(setter);
        assert_eq!(*gotten, TestStruct { value: 243523 });
        assert_eq!(Arc::strong_count(&gotten), 1);
    }

}