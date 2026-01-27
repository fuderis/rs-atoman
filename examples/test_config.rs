#![cfg(feature = "config")]
use atoman::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u32,
    }

    impl ::std::default::Default for Person {
        fn default() -> Self {
            Self {
                name: "Bob".to_owned(),
                age: 23,
            }
        }
    }

    let mut cfg = Config::<Person>::new(".test/person.toml")?;
    println!("{cfg:?}");

    assert_eq!(cfg.name, "Bob");
    assert_eq!(cfg.age, 23);

    cfg.age = 24;
    assert_eq!(cfg.age, 24);

    Ok(())
}
