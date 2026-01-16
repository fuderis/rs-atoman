use crate::prelude::*;
use super::*;

/// The atomic state wrapper
#[derive(Clone)]
pub struct StateWrap<T: Default + Clone + Send + Sync> {
    mutex: Arc<Mutex<Arc<T>>>,
    swap: Arc<ArcSwapAny<Arc<T>>>,
}

/// The atomic state
pub struct State<T: Default + Clone + Send + Sync> {
    wrap: Lazy<Arc<StateWrap<T>>>,
}

impl<T: Default + Clone + Send + Sync> State<T> {
    /// Creates a new state
    pub const fn new() -> Self {
        Self {
            wrap: Lazy::new(
                || {
                    let arc_val = Arc::new(T::default());
                    Arc::new(StateWrap {
                        mutex: Arc::new(Mutex::new(arc_val.clone())),
                        swap: Arc::new(ArcSwapAny::from(arc_val)),
                    })
                }
            )
        }
    }

    /// Returns a locked state guard
    pub fn lock(&self) -> StateGuard<T> {
        StateGuard {
            mutex: self.wrap.mutex.clone(),
            swap: self.wrap.swap.clone(),
            data: self.get_cloned(),
        }
    }

    /// Returns a state value
    pub fn get(&self) -> Arc<T> {
        self.wrap.swap.load_full()
    }

    /// Returns a clone of state value
    pub fn get_cloned(&self) -> T {
        self.wrap.swap.load_full().as_ref().clone()
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

impl<T: Default + Clone + Send + Sync> ::std::default::Default for State<T> {
    fn default() -> Self {
        let this = Self::new();
        this.set(Default::default());
        this
    }
}

impl<T: Default + Clone + Send + Sync> ::std::convert::From<T> for State<T> {
    fn from(value: T) -> Self {
        let this = Self::new();
        this.set(value);
        this
    }
}

impl<T: Default + Clone + Send + Sync + Debugging> ::std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.get())
    }
}

impl<T: Default + Clone + Send + Sync + Displaying> ::std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.get())
    }
}
