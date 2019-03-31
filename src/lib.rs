use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    /// Cannot write
    Write(std::io::Error),
    /// Cannt read
    Read(std::io::Error),
    /// Deserialization error
    TomlRead(toml::de::Error),
    /// Serialization error
    TomlWrite(toml::ser::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Write(err) => write!(f, "cannot write: {}", err),
            Error::Read(err) => write!(f, "cannot read: {}", err),
            Error::TomlRead(err) => write!(f, "toml read error: {}", err),
            Error::TomlWrite(err) => write!(f, "toml write error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Write(err) | Error::Read(err) => Some(err as &(dyn std::error::Error)),
            Error::TomlRead(err) => Some(err as &(dyn std::error::Error)),
            Error::TomlWrite(err) => Some(err as &(dyn std::error::Error)),
        }
    }
}

/// State of the instance from `load_or_default`
#[derive(Debug)]
pub enum LoadState<T> {
    /// When the instance was defaulted
    Default(T),
    /// When the instance was loaded
    Loaded(T),
}

/// Trait to provide easier loaded/saving of a `config` type
///
/// Provide static strs for `Qualifier`, `Organization`, `Application` and `name()`
///
/// Will which will produce $XDG_CONFIG_PATH/qualifier.organization.application/name
///
/// ```rust
/// Qualifier = "com.github",
/// Organization = "museun",
/// Application = "foobar",
/// // ..
/// fn name() -> &'static str { "config.toml" }
///
/// // -> "~/.config/com.github/museun/foobar/config.toml
/// ````
pub trait Configurable: Default + serde::Serialize + serde::de::DeserializeOwned {
    /// Qualifier (e.g. "com.github")
    #[allow(non_upper_case_globals)]
    const Qualifier: &'static str;
    /// Organization (e.g. "museun" (in github.com/museun))
    #[allow(non_upper_case_globals)]
    const Organization: &'static str;
    /// Application (e.g. "foo" (in github.com/museun/foo))
    ///
    /// Defaults to $CARGO_PKG_NAME
    #[allow(non_upper_case_globals)]
    const Application: &'static str = env!("CARGO_PKG_NAME");

    /// The name of the toml file
    fn name() -> &'static str;

    /// Ensures the directory exists
    fn ensure_dir() -> Result<PathBuf, Error> {
        let (qualifier, org, app) = (Self::Qualifier, Self::Organization, Self::Application);
        let dirs = directories::ProjectDirs::from(qualifier, org, app)
            .expect("system must have a valid $HOME directory");
        let dirs = dirs.config_dir();
        fs::create_dir_all(&dirs).map_err(Error::Write)?;
        Ok(dirs.to_owned())
    }

    /// Loads, or defaults the configuration
    ///
    /// Returns a `LoadState`
    /// * Default meant it created a default instance
    /// * Loaded meant it create the instance from the file    
    fn load_or_default() -> Result<LoadState<Self>, Error> {
        let ok = match Self::load() {
            Ok(this) => LoadState::Loaded(this),
            Err(Error::Read(..)) => LoadState::Default(Self::default()),
            Err(err) => return Err(err),
        };
        Ok(ok)
    }

    /// Tries to load the configuration
    fn load() -> Result<Self, Error> {
        let dir = Self::ensure_dir()?.join(Self::name());
        let data = fs::read_to_string(dir).map_err(Error::Read)?;
        Ok(toml::from_str(&data)
            .map_err(Error::TomlRead)
            .unwrap_or_default())
    }

    /// Tries to save the configuration
    fn save(&self) -> Result<(), Error> {
        let dir = Self::ensure_dir()?.join(Self::name());
        let s = toml::to_string_pretty(&self).map_err(Error::TomlWrite)?;
        fs::write(dir, s).map_err(Error::Write)
    }
}

/// Environment var loader which can be overridden by a .env file
pub struct Env;
impl Env {
    /// Get the `key` from the `env`
    ///
    /// This is overridden by the .env file
    pub fn env<Q>(key: Q) -> Option<String>
    where
        Q: Sized + std::hash::Hash + Eq,
        Q: AsRef<std::ffi::OsStr>,
        String: std::borrow::Borrow<Q>,
    {
        Self::load(".env")
            .remove(&key)
            .or_else(|| env::var(key).ok())
    }

    /// Override the environment from the .env file
    ///
    /// This assumes `KEY\s?=\s?"?VAL"?\s?` and turns it into {KEY:VAL}
    ///
    /// Lines beginning with `#` are ignored
    pub fn load<P>(path: P) -> HashMap<String, String>
    where
        P: AsRef<Path>,
    {
        fs::read_to_string(path)
            .map(|data| {
                data.lines()
                    .filter(|s| s.starts_with('#'))
                    .filter_map(|line| {
                        let mut line = line.splitn(2, '=').map(str::trim);
                        Some((line.next()?.into(), line.next()?.into()))
                    })
                    .inspect(|(k, v)| env::set_var(k, v))
                    .collect()
            })
            .unwrap_or_else(|_| env::vars().collect())
    }
}
