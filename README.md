# configurable
[![doc.rs](https://docs.rs/configurable/badge.svg)](https://docs.rs/configurable/latest/configurable/)
[![Crates.io](https://img.shields.io/crates/v/configurable.svg)](https://crates.io/crates/configurable)

This crate provides a set of functions for loading/saving structs to toml files in
OS-accurate locations

## A normal configuration (e.g. saving something to a toml file in CONFIG_DIR)
```rust
use configurable::{Configurable, Config, Data, Error, LoadState};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct MyConfiguration {
    name: String,
    attempts: u32,
    force: bool,
}

impl Default for MyConfiguration {
    fn default() -> Self {
        Self {
            name: "Foobar".into(),
            attempts: 3,
            force: false,
        }
    }
}

impl Config for MyConfiguration {}

impl Configurable for MyConfiguration {
    const ORGANIZATION: &'static str = "museun";
    const APPLICATION: &'static str = "foobar";
    const NAME: &'static str = "config.toml";

    fn ensure_dir() -> Result<std::path::PathBuf, Error> {
        <Self as Config>::ensure_dir()
    }
}
```

## A 'data' configuration (e.g. saving something to a json file in the DATA_DIR)
```rust
use configurable::{Configurable, Config, Data, Error, LoadState};
use serde::{Serialize, Deserialize};
#[derive(Default, Serialize, Deserialize)]
struct MyData {
    #[serde(flatten)]
    data: std::collections::HashMap<String, String>
}


impl Data for MyData {}

impl Configurable for MyData {
    const ORGANIZATION: &'static str = "museun";
    const APPLICATION: &'static str = "foobar";
    const NAME: &'static str = "data.json";

    fn ensure_dir() -> Result<std::path::PathBuf, Error> {
        <Self as Data>::ensure_dir()
    }
}
```

## Loading the data
```rust
fn load_my_stuff() -> Something {
    use configurable::Configuable;
    // this tries to load the configuration ot creates a default instance of it
    match match Something::load_or_default() {
        Ok(data) => data,
        Err(err) => {
            eprintln!("cannot load configuration: {}", err);
            std::process::exit(1)
        }
    } {
        // it was successfully loaded
        configurable::LoadState::Loaded(this) => this,
        // it was defaulted
        configurable::LoadState::Default(this) => {
            eprintln!(
                "a default configuration was created at: {}",
                Something::path().unwrap().display()
            );
            std::process::exit(1)
        }
    }
}
```

License: 0BSD
