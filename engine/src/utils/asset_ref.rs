use crate::utils::{GetMutRef, MutRef};
use std::cell::{Cell, Ref, RefMut};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, LockResult, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct ReadAssetRef<T: Sized> {
    rc: MutRef<T>,
}

impl<T: Sized> ReadAssetRef<T> {
    pub fn new(mut_ref: &MutRef<T>) -> Self {
        Self {
            rc: MutRef::clone(mut_ref),
        }
    }

    pub fn get(&self) -> Ref<'_, T> {
        self.rc.as_ref().borrow()
    }
}

impl<T: Sized> Clone for ReadAssetRef<T> {
    fn clone(&self) -> Self {
        Self {
            rc: MutRef::clone(&self.rc),
        }
    }
}

impl<T: Sized + PartialEq> PartialEq for ReadAssetRef<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.get() == *other.get()
    }
}

impl<T: Sized + Hash> Hash for ReadAssetRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl<T: Sized + Eq> Eq for ReadAssetRef<T> {}
unsafe impl<T: Sized> Send for ReadAssetRef<T> {}
unsafe impl<T: Sized> Sync for ReadAssetRef<T> {}

#[derive(Debug)]
pub struct RwAssetRef<T: Sized> {
    rc: MutRef<T>,
}

impl<T: Sized> RwAssetRef<T> {
    pub fn new(v: T) -> Self {
        Self { rc: GetMutRef(v) }
    }

    pub fn ro_ref(&self) -> ReadAssetRef<T> {
        ReadAssetRef::new(&self.rc)
    }

    pub fn get(&self) -> Ref<'_, T> {
        self.rc.as_ref().borrow()
    }

    pub fn get_mut(&self) -> RefMut<'_, T> {
        self.rc.as_ref().borrow_mut()
    }

    pub fn set(&mut self, value: T) {
        *self.get_mut() = value;
    }
}

impl<T: Sized> Clone for RwAssetRef<T> {
    fn clone(&self) -> Self {
        Self {
            rc: MutRef::clone(&self.rc),
        }
    }
}

impl<T: Sized + PartialEq> PartialEq for RwAssetRef<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.get() == *other.get()
    }
}

impl<T: Sized + Hash> Hash for RwAssetRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl<T: Sized + Default> Default for RwAssetRef<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Sized + Eq> Eq for RwAssetRef<T> {}
unsafe impl<T: Sized> Send for RwAssetRef<T> {}
unsafe impl<T: Sized> Sync for RwAssetRef<T> {}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn read_asset_ref_sees_mutations_from_primary() {
        let rw_ref = RwAssetRef::new(0u32);
        let ro_ref = rw_ref.ro_ref();
        assert_eq!(*rw_ref.get(), 0u32);
        assert_eq!(*ro_ref.get(), 0u32);
        *rw_ref.get_mut() = 12;
        assert_eq!(*rw_ref.get(), 12u32);
        assert_eq!(*ro_ref.get(), 12u32);
    }
}
