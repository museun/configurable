//! This crate provides a set of functions for loading/saving structs to toml files in
//! OS-accurate locations
//!
//! # A normal configuration (e.g. saving something to a toml file in CONFIG_DIR)
//! ```
//! use configurable::{Configurable, Config, Data, Error, LoadState};
//! use serde::{Serialize, Deserialize};
//! #[derive(Serialize, Deserialize)]
//! struct MyConfiguration {
//!     name: String,
//!     attempts: u32,
//!     force: bool,
//! }
//!
//! impl Default for MyConfiguration {
//!     fn default() -> Self {
//!         Self {
//!             name: "Foobar".into(),
//!             attempts: 3,
//!             force: false,
//!         }
//!     }
//! }
//!
//! impl Config for MyConfiguration {}
//!
//! impl Configurable for MyConfiguration {
//!     const ORGANIZATION: &'static str = "museun";
//!     const APPLICATION: &'static str = "foobar";
//!     const NAME: &'static str = "config.toml";
//!
//!     fn ensure_dir() -> Result<std::path::PathBuf, Error> {
//!         <Self as Config>::ensure_dir()
//!     }
//! }
//! ```
//!
//! # A 'data' configuration (e.g. saving something to a json file in the DATA_DIR)
//! ```
//! use configurable::{Configurable, Config, Data, Error, LoadState};
//! use serde::{Serialize, Deserialize};
//! #[derive(Default, Serialize, Deserialize)]
//! struct MyData {
//!     #[serde(flatten)]
//!     data: std::collections::HashMap<String, String>
//! }
//!
//!  
//! impl Data for MyData {}
//!
//! impl Configurable for MyData {
//!     const ORGANIZATION: &'static str = "museun";
//!     const APPLICATION: &'static str = "foobar";
//!     const NAME: &'static str = "data.json";
//!
//!     fn ensure_dir() -> Result<std::path::PathBuf, Error> {
//!         <Self as Data>::ensure_dir()
//!     }
//! }
//! ```
//!
//! # Loading the data
//! ```no_run
//! fn load_my_stuff() -> Something {
//!     use configurable::Configuable;
//!     // this tries to load the configuration ot creates a default instance of it
//!     match match Something::load_or_default() {
//!         Ok(data) => data,
//!         Err(err) => {
//!             eprintln!("cannot load configuration: {}", err);
//!             std::process::exit(1)
//!         }
//!     } {
//!         // it was successfully loaded
//!         configurable::LoadState::Loaded(this) => this,
//!         // it was defaulted
//!         configurable::LoadState::Default(this) => {
//!             eprintln!(
//!                 "a default configuration was created at: {}",
//!                 Something::path().unwrap().display()
//!             );
//!             std::process::exit(1)
//!         }
//!     }
//! }
//! ```

/// State of the instance from `load_or_default`
#[derive(Debug)]
pub enum LoadState<T> {
    /// When the instance was defaulted
    Default(T),
    /// When the instance was loaded
    Loaded(T),
}

mod configurable;
#[doc(inline)]
pub use self::configurable::{Config, Configurable, Data};

mod env;
#[doc(inline)]
pub use self::env::Env;

mod error;
#[doc(inline)]
pub use self::error::Error;
