use crate::prelude::*;

/// The atomic state guard
pub struct StateGuard<'a, T: Clone> {
    pub(super) rw_lock: RwLockWriteGuard<'a, Arc<T>>,
    pub(super) arc_swap: Arc<ArcSwapAny<Arc<T>>>,
    pub(super) data: T,
}

impl<'a, T: Clone> ::std::ops::Drop for StateGuard<'a, T> {
    fn drop(self: &mut Self) {
        let data = Arc::new(self.data.clone());
        
        *self.rw_lock = data.clone();
        self.arc_swap.store(data);
    }
}

impl<'a, T: Clone> ::std::ops::Deref for StateGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T: Clone> ::std::ops::DerefMut for StateGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<'a, T: Clone + ::std::fmt::Debug> ::std::fmt::Debug for StateGuard<'a, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.data)
    }
}

impl<'a, T: Clone + ::std::fmt::Display> ::std::fmt::Display for StateGuard<'a, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.data)
    }
}
