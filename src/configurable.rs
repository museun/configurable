use super::*;

use std::fs;
use std::path::PathBuf;

/// A Configurable type that loads from the equivalent of `$XDG_DATA_HOME`
pub trait Data: Configurable {
    /// Ensures the directory exists
    fn ensure_dir() -> Result<PathBuf, Error> {
        let (qualifier, org, app) = (Self::Qualifier, Self::Organization, Self::Application);
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
        let (qualifier, org, app) = (Self::Qualifier, Self::Organization, Self::Application);
        let dirs = directories::ProjectDirs::from(qualifier, org, app)
            .expect("system must have a valid $HOME directory");
        let dirs = dirs.config_dir();
        fs::create_dir_all(&dirs).map_err(Error::Write)?;
        Ok(dirs.to_owned())
    }
}

/// Trait to provide easier loaded/saving of a `config` type
///
/// Provide static strs for `Qualifier`, `Organization`, `Application` and `Name`
///
/// Will which will produce $CONFIG_PATH/qualifier.organization.application/name
///
/// ```norun
/// // Qualifier = "com.github",
/// // Organization = "museun",
/// // Application = "foobar",
/// // ..
/// fn name() -> &'static str { "config.toml" }
/// // ..
/// // -> "~/.config/com.github/museun/foobar/config.toml
/// ````
pub trait Configurable: Default + serde::Serialize + serde::de::DeserializeOwned {
    /// Qualifier (e.g. "com.github")
    ///
    /// Defaults to `com.github`    
    #[allow(non_upper_case_globals)]
    const Qualifier: &'static str = "com.github";
    /// Organization (e.g. "museun" (in github.com/museun))
    ///
    /// You must provide this
    #[allow(non_upper_case_globals)]
    const Organization: &'static str;
    /// Application (e.g. "foo" (in github.com/museun/foo))
    ///
    /// You must provide this
    #[allow(non_upper_case_globals)]
    const Application: &'static str;
    /// The name of the toml file, with extension
    ///
    /// ex: `config.toml`
    #[allow(non_upper_case_globals)]
    const Name: &'static str;

    /// Ensures the directory exists
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
        let dir = Self::ensure_dir()?.join(Self::Name);
        let data = fs::read_to_string(dir).map_err(Error::Read)?;
        Ok(toml::from_str(&data)
            .map_err(Error::TomlRead)
            .unwrap_or_default())
    }

    /// Tries to save the configuration
    fn save(&self) -> Result<(), Error> {
        let dir = Self::ensure_dir()?.join(Self::Name);
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
        Self::ensure_dir().map(|d| d.join(Self::Name))
    }
}
