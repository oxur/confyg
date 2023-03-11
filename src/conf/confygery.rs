use serde::{de, ser};
use std::fs;
use toml::Value;
use toml::Value::Table;

use super::merger;
use super::options::Options;
use crate::env;
use crate::searchpath::Finder;

#[derive(Clone, Debug)]
pub struct Confygery {
    pub opts: Options,
    pub configs: Vec<String>,
    map: env::KVMap,
    merged: Value,
    toml: String,
}

impl Default for Confygery {
    fn default() -> Self {
        Self::new()
    }
}

impl Confygery {
    pub fn new() -> Confygery {
        let opts = Options::default();
        let proj = opts.project.clone();
        Confygery {
            opts,
            configs: Vec::new(),
            map: env::KVMap::new(&proj),
            merged: toml::from_str("").unwrap(),
            toml: String::new(),
        }
    }

    pub fn with_opts(&mut self, opts: Options) -> &mut Confygery {
        self.opts = opts;
        self
    }

    pub fn add_str<'a>(&'a mut self, content: &str) -> &'a mut Confygery {
        self.configs.push(content.to_string());
        self
    }

    pub fn add_env(&mut self, opts: env::Options) -> &mut Confygery {
        self.map = env::scan(opts.top_level.clone(), opts.sections);
        self.add_str(&self.map.toml())
    }

    pub fn add_file<'a>(&'a mut self, filename: &str) -> &'a mut Confygery {
        let result = Finder::new()
            .add_paths(self.opts.paths.clone())
            .find(filename);
        match result {
            Ok(path) => {
                let content = fs::read_to_string(path).unwrap();
                self.add_str(&content)
            }
            Err(_) => {
                // If the file isn't found, it's not added
                self
            }
        }
    }

    pub fn add_struct<'a, T: ?Sized>(&'a mut self, value: &T) -> &'a mut Confygery
    where
        T: ser::Serialize,
    {
        let content = toml::to_string(value).unwrap();
        self.configs.push(content);
        self
    }

    // Final steps

    fn merge_all(&mut self) -> &mut Confygery {
        if self.configs.is_empty() {
            return self;
        }
        let mut merged: Value = toml::from_str(&self.configs[0]).unwrap();
        for i in 1..self.configs.len() {
            let value = toml::from_str(&self.configs[i]).unwrap();
            merger::merge(&mut merged, &Table(value));
        }
        self.merged = merged;
        self.toml = toml::ser::to_string(&self.merged.clone()).unwrap();
        self
    }

    pub fn build<T>(&mut self) -> Result<T, toml::de::Error>
    where
        T: de::DeserializeOwned,
    {
        self.merge_all();
        toml::from_str(&self.toml)
    }
}
