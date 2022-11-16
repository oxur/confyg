use std::path::Path;

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

    pub fn find(&self, filename: &str) -> String {
        find_file(filename, &self.opts)
    }
}

pub fn find_file(filename: &str, opts: &Options) -> String {
    let path = Path::new(filename);
    if path.exists() {
        return path.to_str().unwrap().to_string();
    };
    for path in opts.paths.iter() {
        let file = Path::new(path).join(filename);
        if file.exists() {
            return file.to_str().unwrap().to_string();
        };
    };
    String::new()
}
