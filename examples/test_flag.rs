use atoman::prelude::*;

static IS_ACTIVE: Flag = Flag::new();

#[tokio::main]
async fn main() {
    assert!(!IS_ACTIVE.get());
    assert!(IS_ACTIVE.is_false());

    IS_ACTIVE.set(true);
    assert!(IS_ACTIVE.get());

    IS_ACTIVE.swap(false).await;
    assert!(!IS_ACTIVE.get());

    IS_ACTIVE.blocking_swap(true);
    assert!(IS_ACTIVE.get());
}
