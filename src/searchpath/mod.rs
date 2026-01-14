//! File path searching across multiple directories.
//!
//! This module provides utilities for locating files across a list of search paths,
//! similar to how shells search for executables in `$PATH`.
//!
//! # Example
//!
//! ```no_run
//! use confyg::searchpath::{Finder, Options};
//!
//! // Search for a config file in multiple locations
//! let file_path = Finder::new()
//!     .add_path("./config")
//!     .add_path("/etc/myapp")
//!     .add_path("/usr/local/etc/myapp")
//!     .find("app.toml")?;
//!
//! println!("Found config at: {}", file_path);
//! # Ok::<(), confyg::searchpath::FinderError>(())
//! ```
//!
//! # Search Behavior
//!
//! - Paths are searched in the order they are added
//! - The first matching file is returned
//! - The current directory (`.`) is included by default
//! - Returns `FinderError::NotFound` if no file is found in any path

pub use errors::FinderError;
pub use finder::{find_file, Finder};
pub use options::Options;

pub mod errors;
pub mod finder;
pub mod options;
