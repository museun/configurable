/// Errors produced by these traits
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
