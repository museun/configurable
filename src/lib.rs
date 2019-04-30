/// State of the instance from `load_or_default`
#[derive(Debug)]
pub enum LoadState<T> {
    /// When the instance was defaulted
    Default(T),
    /// When the instance was loaded
    Loaded(T),
}

mod configurable;
pub use self::configurable::{Config, Configurable, Data};

mod env;
pub use self::env::Env;

mod error;
pub use self::error::Error;
