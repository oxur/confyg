//! # confyg
//!
//! A simple, TOML-based, ENV-enabled library for configuration management.
//!
//! `confyg` provides a flexible way to build application configurations by:
//! - Loading from TOML files
//! - Scanning environment variables
//! - Merging multiple configuration sources
//! - Converting to strongly-typed Rust structures
//!
//! ## Features
//!
//! - **Multiple Sources**: Combine TOML files, environment variables, strings, and Rust structs
//! - **Path Search**: Automatically search multiple directories for configuration files
//! - **Environment Variable Mapping**: Map prefixed environment variables to TOML sections
//! - **Deep Merging**: Intelligently merge configurations with override semantics
//! - **Type Safety**: Deserialize directly into your Rust types using serde
//!
//! ## Quick Start
//!
//! ```rust
//! use confyg::{Confygery, Result};
//! use serde_derive::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     env: String,
//!     database: Database,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Database {
//!     host: String,
//!     port: u16,
//! }
//!
//! fn load_config() -> Result<Config> {
//!     Confygery::new()?
//!         .add_file("default.toml")?
//!         .add_file("local.toml")?
//!         .build()
//! }
//! ```
//!
//! ## Environment Variables
//!
//! Environment variables can be scanned and converted to TOML format using prefixes and sections:
//!
//! ```rust
//! use confyg::{Confygery, env, Result};
//! use serde_derive::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     database: Database,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Database {
//!     host: String,
//!     port: String,
//! }
//!
//! fn load_with_env() -> Result<Config> {
//!     // Scans for MYAPP_DATABASE_HOST, MYAPP_DATABASE_PORT, etc.
//!     let mut env_opts = env::Options::with_top_level("myapp");
//!     env_opts.add_section("database");
//!
//!     Confygery::new()?
//!         .add_file("config.toml")?
//!         .add_env(env_opts)?  // Environment variables override file config
//!         .build()
//! }
//! ```
//!
//! ## Configuration Priority
//!
//! When multiple sources are added, later sources override earlier ones:
//!
//! ```text
//! add_str("...") → add_file("defaults.toml") → add_env(...) → build()
//!   lowest priority                            highest priority
//! ```
//!
//! ## Modules
//!
//! - [`conf`] - Core configuration building and merging
//! - [`env`] - Environment variable scanning and mapping
//! - [`searchpath`] - File path searching
//! - [`errors`] - Error types

pub use conf::confygery::Confygery;
pub use errors::{ConfigError, Result};

pub mod conf;
pub mod env;
pub mod errors;
pub mod searchpath;
