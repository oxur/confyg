use serde::{de, ser};
use std::fs;
use toml::Value;
use toml::Value::Table;

use super::merger;
use super::options::Options;
use crate::env;
use crate::errors::{ConfigError, Result};
use crate::searchpath::Finder;

/// A builder for assembling and merging configuration from multiple sources.
///
/// `Confygery` provides a fluent interface for:
/// - Loading TOML configuration files
/// - Scanning environment variables
/// - Merging multiple configuration sources
/// - Deserializing into strongly-typed Rust structs
///
/// # Configuration Priority
///
/// Sources added later override earlier sources during merging:
/// ```text
/// add_str() → add_file("defaults.toml") → add_env() → build()
/// lowest                                  highest priority
/// ```
///
/// # Example
///
/// ```rust
/// use confyg::{Confygery, conf, env, Result};
/// use serde_derive::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct Config {
///     database: Database,
/// }
///
/// #[derive(Debug, Deserialize)]
/// struct Database {
///     host: String,
///     port: u16,
/// }
///
/// fn load_config() -> Result<Config> {
///     // Configure search paths
///     let mut conf_opts = conf::Options::default();
///     conf_opts.add_path("./config")
///              .add_path("/etc/myapp");
///
///     // Configure environment variable scanning
///     let mut env_opts = env::Options::with_top_level("myapp");
///     env_opts.add_section("database");
///
///     // Build the configuration
///     Confygery::new()?
///         .with_opts(conf_opts)?
///         .add_file("defaults.toml")?    // Base configuration
///         .add_file("production.toml")?  // Environment-specific
///         .add_env(env_opts)?            // Environment overrides
///         .build()
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Confygery {
    opts: Options,
    configs: Vec<String>,
    map: env::KVMap,
    merged: Value,
    toml: String,
}

impl Default for Confygery {
    fn default() -> Self {
        Self::new().expect("failed to create default Confygery")
    }
}

impl Confygery {
    /// Returns a reference to the search path options.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    ///
    /// let confyg = Confygery::new().unwrap();
    /// let paths = confyg.options().paths();
    /// ```
    pub fn options(&self) -> &Options {
        &self.opts
    }

    /// Returns a mutable reference to the search path options.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    ///
    /// let mut confyg = Confygery::new().unwrap();
    /// confyg.options_mut().add_path("./config");
    /// ```
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.opts
    }

    /// Returns the number of configuration sources that have been added.
    ///
    /// This includes TOML strings, files, environment variables, and structs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    ///
    /// let mut confyg = Confygery::new().unwrap();
    /// assert_eq!(confyg.config_count(), 0);
    ///
    /// confyg.add_str("key = 'value'").unwrap();
    /// assert_eq!(confyg.config_count(), 1);
    /// ```
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Creates a new configuration builder with default options.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::TomlParse` if internal initialization fails
    /// (extremely unlikely in practice).
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    ///
    /// let confyg = Confygery::new()?;
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn new() -> Result<Confygery> {
        let opts = Options::default();
        Ok(Confygery {
            map: env::KVMap::new(opts.project()),
            opts,
            configs: Vec::new(),
            merged: toml::from_str("").map_err(|e| ConfigError::TomlParse { source: e })?,
            toml: String::new(),
        })
    }

    /// Sets the search path options for finding configuration files.
    ///
    /// This configures which directories will be searched when using `add_file()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::{Confygery, conf};
    ///
    /// let mut opts = conf::Options::default();
    /// opts.add_path("./config")
    ///     .add_path("/etc/myapp");
    ///
    /// let mut confyg = Confygery::new()?
    ///     .with_opts(opts)?;
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn with_opts(&mut self, opts: Options) -> Result<&mut Confygery> {
        self.opts = opts;
        Ok(self)
    }

    /// Adds a TOML configuration string as a source.
    ///
    /// The TOML content is validated but not parsed until `build()` is called.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    ///
    /// let toml_config = r#"
    ///     env = "development"
    ///     [database]
    ///     host = "localhost"
    ///     port = 5432
    /// "#;
    ///
    /// let mut confyg = Confygery::new()?
    ///     .add_str(toml_config)?;
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn add_str<'a>(&'a mut self, content: &str) -> Result<&'a mut Confygery> {
        self.configs.push(content.to_string());
        Ok(self)
    }

    /// Scans environment variables and adds them as a configuration source.
    ///
    /// Environment variables matching the configured prefix and sections are
    /// converted to TOML format and added to the configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::{Confygery, env};
    /// use std::env as std_env;
    ///
    /// // Set environment variables
    /// std_env::set_var("MYAPP_ENV", "production");
    /// std_env::set_var("MYAPP_DB_HOST", "db.example.com");
    ///
    /// let mut env_opts = env::Options::with_top_level("myapp");
    /// env_opts.add_section("db");
    ///
    /// let mut confyg = Confygery::new()?
    ///     .add_env(env_opts)?;
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn add_env(&mut self, opts: env::Options) -> Result<&mut Confygery> {
        self.map = env::scan(opts.top_level(), opts.sections());
        self.add_str(&self.map.toml())
    }

    /// Adds a TOML configuration file as a source.
    ///
    /// The file is searched for in the configured search paths (see `with_opts`).
    /// If the file is not found in any path, it is silently ignored (not an error).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file is found but cannot be read (permissions, I/O error)
    /// - The file path contains invalid UTF-8
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    ///
    /// let mut confyg = Confygery::new()?
    ///     .add_file("defaults.toml")?
    ///     .add_file("local.toml")?;  // Ignored if not found
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn add_file<'a>(&'a mut self, filename: &str) -> Result<&'a mut Confygery> {
        let result = Finder::new().add_paths(self.opts.paths()).find(filename);
        match result {
            Ok(path) => {
                let content = fs::read_to_string(&path).map_err(|e| ConfigError::FileRead {
                    path: path.clone(),
                    source: e,
                })?;
                self.add_str(&content)
            }
            Err(crate::searchpath::FinderError::NotFound(_)) => {
                // If the file isn't found, it's not added (this is expected behavior)
                Ok(self)
            }
            Err(e) => Err(ConfigError::from(e)),
        }
    }

    /// Adds a serializable Rust struct as a configuration source.
    ///
    /// The struct is serialized to TOML format and added to the configuration.
    /// The struct must implement `serde::Serialize`.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::TomlSerialize` if the struct cannot be serialized to TOML.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Defaults {
    ///     timeout: u32,
    ///     retries: u8,
    /// }
    ///
    /// let defaults = Defaults {
    ///     timeout: 30,
    ///     retries: 3,
    /// };
    ///
    /// let mut confyg = Confygery::new()?
    ///     .add_struct(&defaults)?;
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn add_struct<'a, T>(&'a mut self, value: &T) -> Result<&'a mut Confygery>
    where
        T: ?Sized + ser::Serialize,
    {
        let content =
            toml::to_string(value).map_err(|e| ConfigError::TomlSerialize { source: e })?;
        self.configs.push(content);
        Ok(self)
    }

    // Final steps

    /// Merges all configuration sources.
    ///
    /// This is called internally by `build()`.
    fn merge_all(&mut self) -> Result<()> {
        if self.configs.is_empty() {
            return Err(ConfigError::NoConfigs);
        }
        let mut merged: Value =
            toml::from_str(&self.configs[0]).map_err(|e| ConfigError::TomlParse { source: e })?;
        for config in &self.configs[1..] {
            let value = toml::from_str(config).map_err(|e| ConfigError::TomlParse { source: e })?;
            merger::merge(&mut merged, &Table(value));
        }
        self.merged = merged;
        self.toml = toml::ser::to_string(&self.merged)
            .map_err(|e| ConfigError::TomlSerialize { source: e })?;
        Ok(())
    }

    /// Merges all configuration sources and deserializes into a target type.
    ///
    /// This consumes all added configuration sources (files, strings, environment variables,
    /// structs), merges them with later sources overriding earlier ones, and deserializes
    /// the result into the specified type.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No configuration sources were added (`ConfigError::NoConfigs`)
    /// - A configuration source contains invalid TOML (`ConfigError::TomlParse`)
    /// - The merged configuration doesn't match the target type's structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::Confygery;
    /// use serde_derive::Deserialize;
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct Config {
    ///     app_name: String,
    ///     version: String,
    /// }
    ///
    /// let config: Config = Confygery::new()?
    ///     .add_str(r#"
    ///         app_name = "MyApp"
    ///         version = "1.0.0"
    ///     "#)?
    ///     .build()?;
    ///
    /// assert_eq!(config.app_name, "MyApp");
    /// assert_eq!(config.version, "1.0.0");
    /// # Ok::<(), confyg::ConfigError>(())
    /// ```
    pub fn build<T>(&mut self) -> Result<T>
    where
        T: de::DeserializeOwned,
    {
        self.merge_all()?;
        toml::from_str(&self.toml).map_err(|e| ConfigError::TomlParse { source: e })
    }
}
