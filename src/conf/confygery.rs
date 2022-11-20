use serde::de;
use std::fs;
use toml::Value;
use toml::Value::Table;
use crate::env;
use crate::searchpath::Finder;
use super::options::Options;
use super::merger::merge;

#[derive(Clone, Debug, Default)]
pub struct Confygery {
    pub opts: Options,
    pub configs: Vec<String>,
    map: env::KVMap,
    toml: String,
}

impl Confygery {
    pub fn new() -> Confygery {
        let opts = Options::default();
        let proj = opts.project.clone();
        Confygery {
            opts: opts,
            configs:  Vec::new(),
            map: env::KVMap::new(&proj),
            toml: String::new(),
        }
    }

    pub fn with_opts<'a>(&'a mut self, opts: Options) -> &'a mut Confygery {
        self.opts = opts;
        self
    }

    pub fn add_str<'a>(&'a mut self, content: &str) -> &'a mut Confygery {
        self.configs.push(content.to_string());
        self
    }

    pub fn add_env<'a>(&'a mut self, opts: env::Options) -> &'a mut Confygery {
        self.map = env::scan(opts.top_level.clone(), opts.sections.clone());
        self.add_str(&self.map.toml())
    }

    pub fn add_file<'a>(&'a mut self, filename: &str) -> &'a mut Confygery {
        let path = Finder::new()
            .add_paths(self.opts.paths.clone())
            .find(filename)
            .unwrap();
        let content = fs::read_to_string(path).unwrap();
        self.add_str(&content.to_string())
    }

    // Final steps

    fn merge_all<'a>(&'a mut self) -> &'a mut Confygery {
        let mut merged: Value = toml::from_str(&self.configs[0]).unwrap();
        for i in 1..self.configs.len() {
            let value = toml::from_str(&self.configs[i]).unwrap();
            merge(&mut merged, &Table(value));
        }
        self.toml = merged.to_string();
        self
    }

    pub fn build<'a, T>(&'a mut self) -> Result<T, toml::de::Error>
    where
    T: de::Deserialize<'a>,
    {
        self.merge_all();
        let built = toml::from_str(&self.toml)?;
        Ok(built)
    }
}
