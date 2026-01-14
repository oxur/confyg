/// Configuration options for file path searching.
///
/// Specifies directories to search when loading configuration files
/// and an optional project name prefix.
///
/// # Example
///
/// ```rust
/// use confyg::conf::Options;
///
/// let mut opts = Options::default();
/// opts.add_path("./config")
///     .add_path("/etc/myapp")
///     .set_project("myapp");
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Options {
    paths: Vec<String>,
    project: String,
}

impl Options {
    /// Creates a new `Options` with default values.
    ///
    /// Equivalent to `Options::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Options` with the specified project name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::conf::Options;
    ///
    /// let opts = Options::with_project("myapp");
    /// assert_eq!(opts.project(), "myapp");
    /// ```
    pub fn with_project(project: impl Into<String>) -> Self {
        Self {
            project: project.into(),
            paths: Vec::new(),
        }
    }

    /// Returns the configured search paths.
    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    /// Adds a directory to the search path list.
    ///
    /// Paths are searched in the order they are added.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::conf::Options;
    ///
    /// let mut opts = Options::default();
    /// opts.add_path("./config")
    ///     .add_path("/etc/myapp");
    ///
    /// assert_eq!(opts.paths(), &["./config", "/etc/myapp"]);
    /// ```
    pub fn add_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.paths.push(path.into());
        self
    }

    /// Replaces all search paths with the provided list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::conf::Options;
    ///
    /// let mut opts = Options::default();
    /// opts.set_paths(vec!["./config".to_string(), "/etc/myapp".to_string()]);
    /// ```
    pub fn set_paths(&mut self, paths: Vec<String>) -> &mut Self {
        self.paths = paths;
        self
    }

    /// Returns the configured project name.
    pub fn project(&self) -> &str {
        &self.project
    }

    /// Sets the project name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::conf::Options;
    ///
    /// let mut opts = Options::default();
    /// opts.set_project("myapp");
    ///
    /// assert_eq!(opts.project(), "myapp");
    /// ```
    pub fn set_project(&mut self, project: impl Into<String>) -> &mut Self {
        self.project = project.into();
        self
    }
}
