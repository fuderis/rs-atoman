use atoman::prelude::*;

static CONFIG: State<Config> = State::new();

#[derive(Default, Clone)]
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
