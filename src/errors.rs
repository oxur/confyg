use std::io;
use thiserror::Error;

/// Errors that can occur when building configurations.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// A file could not be read.
    ///
    /// This can occur when:
    /// - The file exists but you lack permissions
    /// - The filesystem returns an I/O error
    /// - The path exists but is not a file (e.g., a directory)
    ///
    /// Note: If a file is not found, it is silently ignored and this
    /// error is not returned.
    #[error("failed to read file '{path}': {source}")]
    FileRead {
        /// The path that could not be read
        path: String,
        /// The underlying I/O error
        #[source]
        source: io::Error,
    },

    /// Failed to parse TOML content.
    ///
    /// This occurs when the TOML syntax is invalid or the structure
    /// doesn't match the expected schema.
    #[error("failed to parse TOML: {source}")]
    TomlParse {
        /// The underlying TOML deserialization error
        #[source]
        source: toml::de::Error,
    },

    /// Failed to serialize to TOML format.
    ///
    /// This can occur when converting Rust structures to TOML strings.
    #[error("failed to serialize to TOML: {source}")]
    TomlSerialize {
        /// The underlying TOML serialization error
        #[source]
        source: toml::ser::Error,
    },

    /// A file was not found during path search.
    #[error("file not found: {0}")]
    FileNotFound(#[from] crate::searchpath::FinderError),

    /// No configurations were added before calling build().
    #[error("no configurations added")]
    NoConfigs,

    /// An invalid configuration state was encountered.
    #[error("invalid configuration state: {0}")]
    InvalidState(String),

    /// A path contained invalid UTF-8.
    #[error("path contains invalid UTF-8: {0}")]
    InvalidPath(String),
}

/// Result type alias using ConfigError.
pub type Result<T> = std::result::Result<T, ConfigError>;
