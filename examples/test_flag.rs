use atoman::prelude::*;

static IS_ACTIVE: Flag = Flag::new();

#[tokio::main]
async fn main() {
    assert_eq!(IS_ACTIVE.get(), false);
    assert!(IS_ACTIVE.is_false());

    IS_ACTIVE.set(true);
    assert_eq!(IS_ACTIVE.get(), true);

    IS_ACTIVE.swap(false).await;
    assert_eq!(IS_ACTIVE.get(), false);
}
