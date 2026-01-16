use crate::prelude::*;

/// The atomic flag wrapper
#[derive(Clone)]
pub struct FlagWrap {
    state: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

/// The atomic flag
pub struct Flag {
    wrap: Lazy<Arc<FlagWrap>>,
}

impl Flag {
    /// Creates a new flag
    pub const fn new() -> Self {
        Self {
            wrap: Lazy::new(
                || Arc::new(FlagWrap {
                    state: Arc::new(AtomicBool::new(false)),
                    notify: Arc::new(Notify::new()),
                })
            )
        }
    }

    /// Check state for 'true'
    pub fn is_true(&self) -> bool {
        self.get()
    }

    /// Check state for 'false'
    pub fn is_false(&self) -> bool {
        !self.get()
    }

    /// Get actual state
    pub fn get(&self) -> bool {
        self.wrap.state.load(Ordering::SeqCst)
    }

    /// Set a new state
    pub fn set(&self, value: bool) {
        self.wrap.state.store(value, Ordering::SeqCst);
        self.wrap.notify.notify_waiters();
    }

    /// Wait state change
    pub async fn wait(&self, value: bool) {
        loop {
            if self.get() == value {
                break;
            }

            self.wrap.notify.notified().await;
        }
    }
    
    /// Wait & swap state
    pub async fn swap(&self, value: bool) {
        self.wait(!value).await;
        self.set(value);
    }
}

impl ::std::default::Default for Flag {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::fmt::Debug for Flag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.get())
    }
}

impl ::std::fmt::Display for Flag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.get())
    }
}

impl ::std::cmp::Eq for Flag {}

impl ::std::cmp::PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl ::std::cmp::PartialEq<bool> for Flag {
    fn eq(&self, other: &bool) -> bool {
        &self.get() == other
    }
}

impl ::std::convert::From<bool> for Flag {
    fn from(value: bool) -> Self {
        let this = Self::new();
        this.set(value);
        this
    }
}

impl ::std::convert::Into<bool> for Flag {
    fn into(self) -> bool {
        self.get()
    }
}
