#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Options {
    paths: Vec<String>,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    pub fn add_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.paths.push(path.into());
        self
    }

    pub fn set_paths(&mut self, paths: Vec<String>) -> &mut Self {
        self.paths = paths;
        self
    }
}
