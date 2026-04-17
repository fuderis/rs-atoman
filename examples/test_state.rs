use atoman::prelude::*;

static CONFIG: State<Config> = State::new();

#[derive(Default, Clone)]
pub struct Config {
    pub count: i32,
}

#[tokio::main]
async fn main() {
    CONFIG.set(Config { count: 10 }).await;
    assert_eq!(CONFIG.get().await.count, 10);

    CONFIG.blocking_set(Config { count: 15 });
    assert_eq!(CONFIG.blocking_get().count, 15);

    CONFIG.dirty_set(Config { count: 16 });
    assert_eq!(CONFIG.dirty_get().count, 16);

    CONFIG.map(|cfg| cfg.count = 20).await;
    assert_eq!(CONFIG.get().await.count, 20);

    CONFIG.lock().await.count = 30;
    assert_eq!(CONFIG.get().await.count, 30);
}
