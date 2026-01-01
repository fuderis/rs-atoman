pub mod state;   pub use state::State;
pub mod guard;   pub use guard::StateGuard;

pub(super) const ERR_MSG: &str = "The data has been poisoned!";
