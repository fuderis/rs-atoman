use crate::prelude::*;
use crate::flag::Flag;
use super::ERR_MSG;

/// The atomic state guard
pub struct StateGuard<T: Clone + Send + Sync> {
    pub(super) mutex: Arc<Mutex<Arc<T>>>,
    pub(super) swap: Arc<ArcSwapAny<Arc<T>>>,
    pub(super) data: T,
    pub(super) lock: Arc<Flag>
}

impl<T: Clone + Send + Sync> ::std::ops::Drop for StateGuard<T> {
    fn drop(self: &mut Self) {
        let data = Arc::new(self.data.clone());
        
        *self.mutex.lock().expect(ERR_MSG) = data.clone();
        self.swap.store(data);
        self.lock.set(false);
    }
}

impl<T: Clone + Send + Sync> ::std::ops::Deref for StateGuard<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Clone + Send + Sync> ::std::ops::DerefMut for StateGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: Clone + Send + Sync + ::std::fmt::Debug> ::std::fmt::Debug for StateGuard<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.data)
    }
}

impl<T: Clone + Send + Sync + ::std::fmt::Display> ::std::fmt::Display for StateGuard<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.data)
    }
}
