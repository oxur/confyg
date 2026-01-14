/// Configuration options for environment variable scanning.
///
/// Specifies the top-level prefix and section prefixes to scan for
/// when converting environment variables to TOML configuration.
///
/// # Environment Variable Format
///
/// Environment variables must follow the pattern: `TOPLEVEL_SECTION_KEY=value`
///
/// - `top_level`: The application-level prefix (e.g., "myapp" → MYAPP_*)
/// - `sections`: Optional section prefixes (e.g., "database" → MYAPP_DATABASE_*)
///
/// # Example
///
/// ```rust
/// use confyg::env::Options;
///
/// let mut opts = Options::with_top_level("myapp");
/// opts.add_section("database")
///     .add_section("cache");
///
/// // Will scan for:
/// // MYAPP_*           → top-level keys
/// // MYAPP_DATABASE_*  → [database] section
/// // MYAPP_CACHE_*     → [cache] section
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Options {
    top_level: String,
    sections: Vec<String>,
}

impl Options {
    /// Creates a new `Options` with default values.
    ///
    /// Equivalent to `Options::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Options` with the specified top-level prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::env::Options;
    ///
    /// let opts = Options::with_top_level("myapp");
    /// assert_eq!(opts.top_level(), "myapp");
    /// ```
    pub fn with_top_level(top_level: impl Into<String>) -> Self {
        Self {
            top_level: top_level.into(),
            sections: Vec::new(),
        }
    }

    /// Returns the configured top-level prefix.
    pub fn top_level(&self) -> &str {
        &self.top_level
    }

    /// Sets the top-level prefix for environment variable scanning.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::env::Options;
    ///
    /// let mut opts = Options::default();
    /// opts.set_top_level("myapp");
    ///
    /// assert_eq!(opts.top_level(), "myapp");
    /// ```
    pub fn set_top_level(&mut self, top_level: impl Into<String>) -> &mut Self {
        self.top_level = top_level.into();
        self
    }

    /// Returns the configured section prefixes.
    pub fn sections(&self) -> &[String] {
        &self.sections
    }

    /// Adds a section prefix to scan for.
    ///
    /// Sections are processed in the order they are added.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::env::Options;
    ///
    /// let mut opts = Options::with_top_level("myapp");
    /// opts.add_section("database")
    ///     .add_section("cache");
    ///
    /// assert_eq!(opts.sections(), &["database", "cache"]);
    /// ```
    pub fn add_section(&mut self, section: impl Into<String>) -> &mut Self {
        self.sections.push(section.into());
        self
    }

    /// Replaces all section prefixes with the provided list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use confyg::env::Options;
    ///
    /// let mut opts = Options::with_top_level("myapp");
    /// opts.set_sections(vec!["database".to_string(), "cache".to_string()]);
    /// ```
    pub fn set_sections(&mut self, sections: Vec<String>) -> &mut Self {
        self.sections = sections;
        self
    }
}
