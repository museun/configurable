use std::collections::HashMap;
use std::env;
use std::path::Path;

/// Environment var loader which can be overridden by a .env file
pub struct Env;
impl Env {
    /// Get the `key` from the `env`
    ///
    /// This is overridden by the .env file
    pub fn env(key: &str) -> Option<String> {
        Self::load(".env")
            .remove(key)
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
        std::fs::read_to_string(path)
            .map(|data| {
                data.lines()
                    .filter(|s| !s.starts_with('#'))
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
