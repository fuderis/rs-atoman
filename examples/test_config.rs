use atoman::prelude::*;
#[cfg(any(feature = "json-config", feature = "toml-config"))]
use serde::{ Serialize, Deserialize };

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(any(feature = "json-config", feature = "toml-config"))]
    {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u32
        }
        
        impl ::std::default::Default for Person {
            fn default() -> Self {
                Self {
                    name: "Bob".to_owned(),
                    age: 23
                }
            }
        }

        let cfg = Config::<Person>::new(".test/person.toml")?;
    
        assert_eq!(cfg.get().name, "Bob");
        assert_eq!(cfg.get().age, 23);

        cfg.lock().age = 24;
        assert_eq!(cfg.get().age, 24);
    }

    Ok(())
}
