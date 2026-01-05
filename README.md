[![github]](https://github.com/fuderis/rs-atoman)&ensp;
[![crates-io]](https://crates.io/crates/atoman)&ensp;
[![docs-rs]](https://docs.rs/atoman)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Atoman (Atomic State)

Atoman is a Rust library for safe concurrent access to static asynchronous data across your application. It provides atomic flags and state wrappers with async setters, getters, and locking mechanisms, bridging synchronous static variables with async runtimes like Tokio.


## Features:

* Global configuration management with file-based persistence (`TOML/JSON`).
* Structured logging with async-safe output to files.
* Feature flags and shared state in async applications.
* Data is stored in `Arc` for zero-copy access and thread safety.


## Examples:

### Atomic Flag:
```rust
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
```

### Atomic State:
```rust
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
```

### Config:
```rust
use atoman::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Person {
    name: String,
    age: u32
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Bob".to_owned(),
            age: 23
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::<Person>::new(".test/person.toml")?;

    assert_eq!(cfg.get().name, "Bob");
    assert_eq!(cfg.get().age, 23);

    cfg.lock().name = 24;
    assert_eq!(cfg.get().age, 24);

    Ok(())
}
```

### Logger:
```rust
use atoman::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(".test/logs", 20)?;
        
    info!("Hello, World!");

    Ok(())
}
```

## Feedback:

> This library distributed under the [MIT](https://github.com/fuderis/rs-atoman/blob/main/LICENSE.md) license.

You can contact me via GitHub or send a message to my telegram [@fuderis](https://t.me/fuderis).
This library is actively evolving, and your suggestions and feedback are always welcome!
