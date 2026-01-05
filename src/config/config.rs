use crate::prelude::*;
use std::{ fs, path::PathBuf };
use serde::{ Serialize, de::DeserializeOwned };

/// The atomic config wrapper
#[derive(Default, Clone)]
pub struct Config<T: Default + Debugging + Clone + Serialize + DeserializeOwned + Send + Sync + 'static> {
    path: PathBuf,
    data: T,
}

impl<T> Config<T>
where
    T: Clone + Default + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static
{
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
            path,
            data,
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
    pub fn write<P: Into<PathBuf>>(&mut self, file_path: P) -> Result<()> {
        self.path = file_path.into();

        // serialize to .toml string:
        let contents = match self.path.extension()
            .map(|s| s.to_str().unwrap_or("TOML"))
            .unwrap_or("TOML")
            .to_uppercase()
            .as_ref()
        {
            #[cfg(any(feature = "toml-config"))]
            "TOML" => toml::to_string_pretty(&self.data).expect("Failed to serialize TOML"),

            #[cfg(any(feature = "json-config"))]
            "JSON" => serde_json::to_string_pretty(&self.data).expect("Failed to serialize JSON"),

            ext => return Err(Error::ConfigExt(ext.to_owned()).into())
        };
        
        // create dir:
        if let Some(parent_dir) = self.path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        
        // write file:
        fs::write(&self.path, contents)?;
        
        Ok(())
    }
    
    /// Updates the config file
    pub fn save(&mut self) -> Result<()> {
        self.write(self.path.clone())
    }

    /// Updates the struct data from config file
    pub fn update(&mut self) -> Result<()> {
        let cfg = Self::read(&self.path)?;
        self.data = cfg.data;

        Ok(())
    }
}

impl<T: Clone + Default + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static> ::std::ops::Deref for Config<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Clone + Default + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static> ::std::ops::DerefMut for Config<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: Clone + Default + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static>  ::std::fmt::Debug for Config<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.data)
    }
}

impl<T: Clone + Default + std::fmt::Display + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static> ::std::fmt::Display for Config<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.data)
    }
}

impl<T: Clone + Default + Debugging + Serialize + DeserializeOwned + Eq + Send + Sync + 'static> ::std::cmp::Eq for Config<T> {}

impl<T: Clone + Default + Debugging + Serialize + DeserializeOwned + PartialEq + Send + Sync + 'static> ::std::cmp::PartialEq for Config<T> {
    fn eq(&self, other: &Self) -> bool {
        &self.data == &other.data
    }
}

impl<T: Clone + Default + Debugging + Serialize + DeserializeOwned + Send + Sync + 'static> ::std::convert::From<T> for Config<T> {
    fn from(value: T) -> Self {
        Self {
            path: Default::default(),
            data: value
        }
    }
}
