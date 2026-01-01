use atoman::prelude::*;

static IS_ACTIVE: Lazy<Flag> = lazy_flag!(false);

#[tokio::main]
async fn main() {
    assert_eq!(*IS_ACTIVE, false);
    assert!(IS_ACTIVE.is_false());

    IS_ACTIVE.set(true);
    assert_eq!(*IS_ACTIVE, true);

    IS_ACTIVE.swap(false).await;
    assert_eq!(*IS_ACTIVE, false);
}
