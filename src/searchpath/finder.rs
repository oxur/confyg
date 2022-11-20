use std::path::Path;

use super::errors::FinderError;

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub paths: Vec<String>,
}

impl Options {
    pub fn default() -> Options {
        Options{
            .. Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Finder {
    pub opts: Options,
}

impl Finder {
    pub fn new() -> Finder {
        Finder {
            opts: Options::default(),
        }
    }

    pub fn add_path<'a>(&'a mut self, path: &str) -> &'a mut Finder {
        self.opts.paths.push(path.to_string());
        self
    }

    pub fn add_paths<'a>(&'a mut self, paths: Vec<String>) -> &'a mut Finder {
        for path in paths {
            self.opts.paths.push(path);
        }
        self
    }

    pub fn find(&self, filename: &str) -> Result<String, FinderError> {
        find_file(filename, &self.opts)
    }
}

pub fn find_file(filename: &str, opts: &Options) -> Result<String, FinderError> {
    let path = Path::new(filename);
    if path.exists() {
        return Ok(path.to_str().unwrap().to_string());
    };
    for path in opts.paths.iter() {
        let file = Path::new(path).join(filename);
        if file.exists() {
            return Ok(file.to_str().unwrap().to_string());
        };
    };
    Err(FinderError::NotFound(filename.to_string()))
}
