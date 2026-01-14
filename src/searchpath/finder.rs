use std::path::Path;

use super::errors::FinderError;
use super::options::Options;

/// A builder for finding files across multiple search paths.
///
/// `Finder` provides a fluent interface for configuring and executing
/// file searches across a list of directories.
///
/// # Search Order
///
/// 1. First checks if the filename exists as-is (absolute or relative path)
/// 2. Then searches each configured path in order
/// 3. Returns the first match found
///
/// # Example
///
/// ```no_run
/// use confyg::searchpath::Finder;
///
/// let result = Finder::new()
///     .add_path("./config")
///     .add_path("/etc/myapp")
///     .find("app.toml")?;
///
/// println!("Found: {}", result);
/// # Ok::<(), confyg::searchpath::FinderError>(())
/// ```
#[derive(Clone, Debug, Default)]
pub struct Finder {
    opts: Options,
}

impl Finder {
    /// Creates a new file finder with default options.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::searchpath::Finder;
    ///
    /// let finder = Finder::new();
    /// ```
    pub fn new() -> Finder {
        Finder {
            opts: Options::default(),
        }
    }

    /// Returns a reference to the search options.
    pub fn options(&self) -> &Options {
        &self.opts
    }

    /// Returns a mutable reference to the search options.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.opts
    }

    /// Adds a directory to the search path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::searchpath::Finder;
    ///
    /// let mut finder = Finder::new();
    /// finder.add_path("./config")
    ///       .add_path("/etc/myapp");
    /// ```
    pub fn add_path<'a>(&'a mut self, path: &str) -> &'a mut Finder {
        self.opts.add_path(path);
        self
    }

    /// Replaces all search paths with the provided list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::searchpath::Finder;
    ///
    /// let paths = vec!["./config".to_string(), "/etc/myapp".to_string()];
    /// let mut finder = Finder::new();
    /// finder.add_paths(&paths);
    /// ```
    pub fn add_paths(&mut self, paths: &[String]) -> &mut Finder {
        self.opts.set_paths(paths.to_vec());
        self
    }

    /// Searches for a file across the configured paths.
    ///
    /// Returns the full path to the first matching file found.
    ///
    /// # Errors
    ///
    /// - `FinderError::NotFound` if the file is not found in any path
    /// - `FinderError::PathConversion` if the path contains invalid UTF-8
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::searchpath::Finder;
    ///
    /// let path = Finder::new()
    ///     .add_path("./examples")
    ///     .find("env.rs")?;
    ///
    /// assert!(path.ends_with("env.rs"));
    /// # Ok::<(), confyg::searchpath::FinderError>(())
    /// ```
    pub fn find(&self, filename: &str) -> Result<String, FinderError> {
        find_file(filename, &self.opts)
    }
}

/// Finds a file across multiple search paths.
///
/// This is a convenience function that searches for a file using the provided options.
/// For a builder-style interface, use [`Finder`] instead.
///
/// # Search Behavior
///
/// 1. First checks if `filename` exists as-is (absolute or relative path)
/// 2. Then searches each path in `opts.paths()` in order
/// 3. Returns the first match found
///
/// # Errors
///
/// - `FinderError::NotFound` if the file is not found in any path
/// - `FinderError::PathConversion` if a path contains invalid UTF-8
///
/// # Example
///
/// ```rust
/// use confyg::searchpath::{find_file, Options};
///
/// let mut opts = Options::default();
/// opts.add_path("./examples");
///
/// let path = find_file("env.rs", &opts)?;
/// assert!(path.ends_with("env.rs"));
/// # Ok::<(), confyg::searchpath::FinderError>(())
/// ```
pub fn find_file(filename: &str, opts: &Options) -> Result<String, FinderError> {
    let path = Path::new(filename);
    if path.exists() {
        return path
            .to_str()
            .ok_or_else(|| FinderError::PathConversion(filename.to_string()))
            .map(|s| s.to_string());
    }
    for search_path in opts.paths().iter() {
        let file = Path::new(search_path).join(filename);
        if file.exists() {
            return file
                .to_str()
                .ok_or_else(|| FinderError::PathConversion(file.to_string_lossy().into_owned()))
                .map(|s| s.to_string());
        }
    }
    Err(FinderError::NotFound(filename.to_string()))
}
