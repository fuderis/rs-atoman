use crate::prelude::*;
use std::time::{Duration, Instant};

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
            wrap: Lazy::new(|| {
                Arc::new(FlagWrap {
                    state: Arc::new(AtomicBool::new(false)),
                    notify: Arc::new(Notify::new()),
                })
            }),
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

    /// Wait for state change
    pub async fn wait(&self, value: bool) {
        while self.get() != value {
            self.wrap.notify.notified().await;
        }
    }

    /// Wait for state change (with synchronously blocking)
    pub fn blocking_wait(&self, value: bool) {
        while self.get() != value {
            std::thread::yield_now();
        }
    }

    /// Wait for state change by interval (with synchronously blocking)
    pub fn blocking_wait_timeout(&self, value: bool, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;

        while self.get() != value {
            if Instant::now() > deadline {
                return false;
            }
            std::thread::yield_now();
        }
        true
    }

    /// Wait & swap flag
    pub async fn swap(&self, value: bool) {
        self.wait(!value).await;
        self.set(value);
    }

    /// Wait & swap flag (with synchronously blocking)
    pub fn blocking_swap(&self, value: bool) {
        self.blocking_wait(!value);
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

#[allow(clippy::from_over_into)]
impl ::std::convert::Into<bool> for Flag {
    fn into(self) -> bool {
        self.get()
    }
}
