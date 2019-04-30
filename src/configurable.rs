use super::*;

use std::fs;
use std::path::PathBuf;

/// A Configurable type that loads from the equivalent of `$XDG_DATA_HOME`
pub trait Data: Configurable {
    /// Ensures the directory exists
    fn ensure_dir() -> Result<PathBuf, Error> {
        let (qualifier, org, app) = (Self::QUALIFIER, Self::ORGANIZATION, Self::APPLICATION);
        let dirs = directories::ProjectDirs::from(qualifier, org, app)
            .expect("system must have a valid $HOME directory");
        let dirs = dirs.data_dir();
        fs::create_dir_all(&dirs).map_err(Error::Write)?;
        Ok(dirs.to_owned())
    }
}

/// A Configurable type that loads from the equivalent of `$XDG_CONFIG_HOME`
pub trait Config: Configurable {
    /// Ensures the directory exists
    fn ensure_dir() -> Result<PathBuf, Error> {
        let (qualifier, org, app) = (Self::QUALIFIER, Self::ORGANIZATION, Self::APPLICATION);
        let dirs = directories::ProjectDirs::from(qualifier, org, app)
            .expect("system must have a valid $HOME directory");
        let dirs = dirs.config_dir();
        fs::create_dir_all(&dirs).map_err(Error::Write)?;
        Ok(dirs.to_owned())
    }
}

/// Trait to provide easier loaded/saving of a `config` type
///
/// Provide static strs for `QUALIFIER`, `ORGANIZATION`, `APPLICATION` and `NAME`
///
/// Will which will produce $CONFIG_PATH/qualifier.organization.application/name
///
/// # Configuration-style configs (e.g. stuff that should be human editable)
/// ```
/// use serde::{Serialize, Deserialize};
/// use std::path::PathBuf;
/// use configurable::{Config, Data, Configurable, Error};
/// 
/// // Default is required
/// #[derive(Default, Serialize, Deserialize)]
/// struct MyConfig;
/// 
/// // For configurations (e.g. foo.toml)
/// impl Config for MyConfig {};
/// impl Configurable for MyConfig {
///     const ORGANIZATION: &'static str = "museun";
///     const APPLICATION: &'static str = "foobar";
///     const NAME: &'static str = "config.toml";
///
///     fn ensure_dir() -> Result<PathBuf, Error> {
///         // Config `configs`
///         <Self as Config>::ensure_dir()
///     }
/// }
/// // will place it here:
/// // -> "~/.config/com.github/museun/foobar/config.toml
/// ```
/// 
/// # Data-style configurations (e.g. formats outside of toml)
/// ```
/// use serde::{Serialize, Deserialize};
/// use std::path::PathBuf;
/// use configurable::{Config, Data, Configurable, Error};
/// 
/// // Default is required
/// #[derive(Default, Serialize, Deserialize)]
/// struct MyMap { map: std::collections::HashMap<String,i32> }
/// 
/// // For configurations (e.g. foo.toml)
/// impl Data for MyMap {};
/// impl Configurable for MyMap {
///     const ORGANIZATION: &'static str = "museun";
///     const APPLICATION: &'static str = "foobar";
///     const NAME: &'static str = "mapping.json";
///
///     fn ensure_dir() -> Result<PathBuf, Error> {
///         // Data `configs`
///         <Self as Data>::ensure_dir()
///     }
/// }
/// // will place it here:
/// // -> "~/.local/share/com.github/museun/foobar/mapping.json
/// ````
pub trait Configurable: Default + serde::Serialize + serde::de::DeserializeOwned {
    /// Qualifier (e.g. "com.github")
    ///
    /// Defaults to `com.github`    
    const QUALIFIER: &'static str = "com.github";
    /// Organization (e.g. "museun" (in github.com/museun))
    ///
    /// You must provide this
    const ORGANIZATION: &'static str;
    /// Application (e.g. "foo" (in github.com/museun/foo))
    ///
    /// You must provide this
    const APPLICATION: &'static str;
    /// The name of the toml file, with extension
    ///
    /// ex: `config.toml`
    const NAME: &'static str;

    /// Ensures the directory exists
    ///
    /// Implement either `Config` or `Data`
    /// then delegate to it
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use std::path::PathBuf;
    /// # use configurable::{Config, Configurable, Error};
    /// # #[derive(Default, Serialize, Deserialize)]
    /// # struct Foo;
    /// // Config or Data
    /// impl Config for Foo {};
    /// impl Configurable for Foo {
    ///     const ORGANIZATION: &'static str = "some_org";
    ///     const APPLICATION: &'static str = "foobar";
    ///     const NAME: &'static str = "config.toml";
    ///
    ///     fn ensure_dir() -> Result<PathBuf, Error> {
    ///         // Config or Data
    ///         <Self as Config>::ensure_dir()
    ///     }
    /// }
    /// ```
    fn ensure_dir() -> Result<PathBuf, Error>;

    /// Loads, or defaults the configuration
    ///
    /// Returns a `LoadState`
    /// * Default meant it created a default instance
    /// * Loaded meant it create the instance from the file    
    fn load_or_default() -> Result<LoadState<Self>, Error> {
        match Self::load() {
            Ok(this) => Ok(LoadState::Loaded(this)),
            Err(Error::Read(..)) => Ok(LoadState::Default(Self::default())),
            Err(err) => Err(err),
        }
    }

    /// Tries to load the configuration
    fn load() -> Result<Self, Error> {
        let dir = Self::ensure_dir()?.join(Self::NAME);
        let data = fs::read_to_string(dir).map_err(Error::Read)?;
        Ok(toml::from_str(&data)
            .map_err(Error::TomlRead)
            .unwrap_or_default())
    }

    /// Tries to save the configuration
    fn save(&self) -> Result<(), Error> {
        let dir = Self::ensure_dir()?.join(Self::NAME);
        let s = toml::to_string_pretty(&self).map_err(Error::TomlWrite)?;
        fs::write(dir, s).map_err(Error::Write)
    }

    /// Ensures the directory exists and returns a `PathBuf` to it
    fn dir() -> Result<PathBuf, Error> {
        Self::ensure_dir()
    }

    /// Ensures the directory exists and returns a `PathBuf` to the
    /// configuration file inside of the directory
    fn path() -> Result<PathBuf, Error> {
        Self::ensure_dir().map(|d| d.join(Self::NAME))
    }
}
