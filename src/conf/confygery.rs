use serde::de;
use crate::env;
use super::options::Options;

#[derive(Clone, Debug, Default)]
pub struct Confygery<T> {
    pub opts: Options,
    pub configs: Vec<String>,
    map: env::KVMap,
    pub sources: Vec<T>,
    pub errors: Vec<toml::de::Error>,
}

impl<T: Clone> Confygery<T> {
    pub fn new() -> Confygery<T> {
        let opts = Options::default();
        let proj = opts.project.clone();
        Confygery {
            opts: opts,
            configs:  Vec::new(),
            map: env::KVMap::new(&proj),
            sources: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_str<'a>(&'a mut self, content: &str) -> &'a mut Confygery<T> {
        self.configs.push(content.to_string());
        self
    }

    pub fn add_source<'a>(&'a mut self, data: T) -> &'a mut Confygery<T> {
        self.sources.push(data);
        self
    }

    pub fn add_error<'a>(&'a mut self, e: toml::de::Error) -> &'a mut Confygery<T> {
        self.errors.push(e);
        self
    }

    pub fn use_env<'a>(&'a mut self, opts: env::Options) -> &'a mut Confygery<T> {
        self.map = env::scan(opts.top_level.clone(), opts.sections.clone());
        self.add_str(&self.map.toml())
    }

    // Final step

    pub fn build<'a>(&'a mut self) -> &'a mut Confygery<T>
    where
    T: de::Deserialize<'a>,
    {
        // TODO: this is just doing the first one ... let's figure out
        // how to:
        // 1. merge many
        // 2. support env toml here, too
        let c = self.configs[0].to_owned().as_str();
        match toml::from_str::<T>(c) {
            Ok(r) => self.add_source(r.clone()),
            Err(e) => self.add_error(e.clone()),
        };
        self
    }
}
