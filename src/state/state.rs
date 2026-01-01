use crate::prelude::*;
use super::*;

/// The atomic state
#[derive(Clone)]
pub struct State<T: Clone> {
    rw_lock: Arc<RwLock<Arc<T>>>,
    arc_swap: Arc<ArcSwapAny<Arc<T>>>,
}

impl<T: Clone> State<T> {
    /// Creates a new state
    pub fn new(value: T) -> Self {
        let arc_val = Arc::new(value);
        
        Self {
            rw_lock: Arc::new(RwLock::new(arc_val.clone())),
            arc_swap: Arc::new(ArcSwapAny::from(arc_val)),
        }
    }

    /// Returns a locked state guard
    pub fn lock(&self) -> StateGuard<'_, T> {
        let rw_lock = self.rw_lock.write().expect(ERR_MSG);
        let data = (**rw_lock).clone();
        
        StateGuard {
            rw_lock,
            arc_swap: self.arc_swap.clone(),
            data,
        }
    }

    /// Returns a state value
    pub fn get(&self) -> Arc<T> {
        self.arc_swap.load_full()
    }

    /// Returns a clone of state value
    pub fn get_cloned(&self) -> T {
        self.arc_swap.load_full().as_ref().clone()
    }

    /// Sets a new value to state
    pub fn set(&self, value: T) {
        *self.lock() = value;
    }

    /// Writes data directly
    pub fn map(&self, f: impl FnOnce(&mut T)) {
        let mut rw_lock = self.lock();
        let mut data = (*rw_lock).clone();

        f(&mut data);
        *rw_lock = data;
    }
}

impl<T: Clone + Default> ::std::default::Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Clone + Debugging> ::std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.get())
    }
}
