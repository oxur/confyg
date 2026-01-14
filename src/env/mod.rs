//! Environment variable scanning and mapping to TOML configuration.
//!
//! This module provides utilities for reading environment variables with specific
//! prefixes and converting them into TOML-compatible key-value structures.
//!
//! # Environment Variable Format
//!
//! Environment variables are expected to follow this naming pattern:
//! ```text
//! TOPLEVEL_SECTION_KEY=value
//! ```
//!
//! For example, with top-level `myapp` and section `database`:
//! - `MYAPP_DATABASE_HOST` → `[database] host = "..."`
//! - `MYAPP_DATABASE_PORT` → `[database] port = "..."`
//! - `MYAPP_LOG_LEVEL` → `log_level = "..."` (top-level, no section)
//!
//! # Example
//!
//! ```rust
//! use confyg::env::{Scanner, Options};
//! use std::env;
//!
//! // Set some environment variables
//! env::set_var("MYAPP_ENV", "production");
//! env::set_var("MYAPP_DB_HOST", "localhost");
//! env::set_var("MYAPP_DB_PORT", "5432");
//!
//! // Scan and convert to TOML
//! let toml = Scanner::new()
//!     .set_top_level("myapp")
//!     .add_section("db")
//!     .scan()
//!     .toml();
//!
//! // Results in TOML like:
//! // env = 'production'
//! //
//! // [db]
//! // host = 'localhost'
//! // port = '5432'
//! ```
//!
//! # Naming Conventions
//!
//! - Environment variable names are converted to lowercase
//! - Hyphens in section names become underscores (e.g., `my-app` → `MY_APP`)
//! - The prefix is stripped from the final key name
//! - Keys are sorted alphabetically within each section

pub use model::{KVMap, KV};
pub use options::Options;
pub use scanner::{scan, Scanner};

pub mod model;
pub mod options;
pub mod scanner;
