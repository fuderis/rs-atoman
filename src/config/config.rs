use crate::prelude::*;
use std::{ fs, path::PathBuf };
use serde::{ Serialize, de::DeserializeOwned };

/// The atomic config wrapper
#[derive(Clone, Default)]
pub struct Config<T: Default + Debugging + Clone + Serialize + DeserializeOwned + Send + Sync + 'static> {
    path: State<PathBuf>,
    data: State<T>,
}

impl<T> Config<T>
where
    T: Clone + Default + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static
{
    /// Returns the config data
    pub fn get(&self) -> Arc<T> {
        self.data.get()
    }

    /// Locks the config data
    pub fn lock(&self) -> StateGuard<'_, T> {
        self.data.lock()
    }
    
    /// Reads the config file or creates the default
    pub fn new<P: Into<PathBuf>>(file_path: P) -> Result<Self> {
        let file_path = file_path.into();

        // reading the config file:
        let this = if file_path.is_file() {
            Self::read(&file_path)?
        }
        // writing the default config file:
        else {
            let mut this = Config::<T>::default();
            this.write(file_path)?;
            this
        };

        Ok(this)
    }

    /// Parses the config from a raw text
    pub fn parse<P: Into<PathBuf>>(file_path: P, contents: &str) -> Result<Self> {
        let path = file_path.into();

        let data: T = match path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("TOML")
            .to_uppercase()
            .as_str()
        {
            #[cfg(any(feature = "toml-config"))]
            "TOML" => toml::from_str(contents)?,

            #[cfg(any(feature = "json-config"))]
            "JSON" => serde_json::from_str(contents)?,

            ext => return Err(Error::ConfigExt(ext.to_owned()).into()),
        };

        Ok(Config {
            path: State::new(path),
            data: State::new(data),
        })
    }

    /// Reads the config file
    pub fn read<P: Into<PathBuf>>(file_path: P) -> Result<Self> {
        let file_path = file_path.into();

        // reading the config file:
        let contents = fs::read_to_string(&file_path)?;
        Self::parse(file_path, &contents)
    }

    /// Saves the config to custom file path
    pub fn write<P: Into<PathBuf>>(&self, file_path: P) -> Result<()> {
        self.path.set(file_path.into());
        let path = self.path.get_cloned();

        // serialize to .toml string:
        let contents = match path.extension()
            .map(|s| s.to_str().unwrap_or("TOML"))
            .unwrap_or("TOML")
            .to_uppercase()
            .as_ref()
        {
            #[cfg(any(feature = "toml-config"))]
            "TOML" => toml::to_string_pretty(self.data.get().as_ref()).expect("Failed to serialize TOML"),

            #[cfg(any(feature = "json-config"))]
            "JSON" => serde_json::to_string_pretty(self.data.get().as_ref()).expect("Failed to serialize JSON"),

            ext => return Err(Error::ConfigExt(ext.to_owned()).into())
        };
        
        // create dir:
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        
        // write file:
        fs::write(path, contents)?;
        
        Ok(())
    }
    
    /// Updates the config file
    pub fn save(&self) -> Result<()> {
        self.write(&self.path.get_cloned())
    }

    /// Updates the struct data from config file
    pub fn update(&self) -> Result<()> {
        let cfg = Self::read(self.path.get_cloned())?;
        let arc = cfg.data.get();
        drop(cfg);

        self.data.set(Arc::try_unwrap(arc).unwrap());

        Ok(())
    }
}
