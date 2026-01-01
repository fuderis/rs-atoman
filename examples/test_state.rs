use atoman::prelude::*;

static CONFIG: Lazy<State<Config>> = lazy_state!(
    Config {
        count: 0,
    }
);

#[derive(Debug, Clone)]
pub struct Config {
    pub count: i32,
}

fn main() {
    CONFIG.set(Config { count: 10, });
    assert_eq!(CONFIG.get().count, 10);

    CONFIG.map(|cfg| cfg.count = 20);
    assert_eq!(CONFIG.get().count, 20);
    
    CONFIG.lock().count = 30;
    assert_eq!(CONFIG.get().count, 30);
}
