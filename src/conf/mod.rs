//! Core configuration building and merging functionality.
//!
//! This module provides the [`Confygery`] builder for assembling configurations
//! from multiple sources, along with the internal merging logic.
//!
//! # Example
//!
//! ```rust
//! use confyg::{Confygery, conf, Result};
//! use serde_derive::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! struct AppConfig {
//!     name: String,
//!     version: String,
//! }
//!
//! fn load() -> Result<AppConfig> {
//!     let mut opts = conf::Options::default();
//!     opts.add_path("./config")
//!         .add_path("/etc/myapp");
//!
//!     Confygery::new()?
//!         .with_opts(opts)?
//!         .add_str(r#"
//!             name = "myapp"
//!             version = "1.0.0"
//!         "#)?
//!         .build()
//! }
//! # let _ = load();
//! ```

pub use options::Options;

pub mod confygery;
pub mod merger;
pub mod options;
