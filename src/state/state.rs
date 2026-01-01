use crate::prelude::*;
use super::*;

/// The atomic state
#[derive(Clone)]
pub struct State<T: Clone> {
    mutex: Arc<Mutex<Arc<T>>>,
    swap: Arc<ArcSwapAny<Arc<T>>>,
}

impl<T: Clone> State<T> {
    /// Creates a new state
    pub fn new(value: T) -> Self {
        let arc_val = Arc::new(value);
        
        Self {
            mutex: Arc::new(Mutex::new(arc_val.clone())),
            swap: Arc::new(ArcSwapAny::from(arc_val)),
        }
    }

    /// Returns a locked state guard
    pub fn lock(&self) -> StateGuard<'_, T> {
        let mutex = self.mutex.lock().expect(ERR_MSG);
        let data = (**mutex).clone();
        
        StateGuard {
            mutex,
            swap: self.swap.clone(),
            data,
        }
    }

    /// Returns a state value
    pub fn get(&self) -> Arc<T> {
        self.swap.load_full()
    }

    /// Returns a clone of state value
    pub fn get_cloned(&self) -> T {
        self.swap.load_full().as_ref().clone()
    }

    /// Sets a new value to state
    pub fn set(&self, value: T) {
        *self.lock() = value;
    }

    /// Writes data directly
    pub fn map(&self, f: impl FnOnce(&mut T)) {
        let mut mutex = self.lock();
        let mut data = (*mutex).clone();

        f(&mut data);
        *mutex = data;
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
