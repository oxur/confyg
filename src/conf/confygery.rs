use serde::de;
use crate::env;
use super::options::Options;

#[derive(Clone, Debug, Default)]
pub struct Confygery {
    pub opts: Options,
    pub configs: Vec<String>,
    map: env::model::KVMap,
}

impl Confygery {
    pub fn new() -> Confygery {
        Confygery {
            opts: Options::default(),

            .. Default::default()
        }
    }

    pub fn add_str<'a>(&'a mut self, content: &str) -> &'a mut Confygery {
        self.configs.push(content.to_string());
        self
    }

    pub fn use_env<'a>(&'a mut self, opts: env::Options) -> &'a mut Confygery {
        self.map = env::scan(opts.top_level.clone(), opts.sections.clone());
        self.add_str(&self.map.toml())
    }

    // Final step

    pub fn build<'de, T>(&'de self) -> Result<T, toml::de::Error>
    where
    T: de::Deserialize<'de>,
    {
        // TODO: this is just doing the first one ... let's figure out
        // how to:
        // 1. merge many
        // 2. support env toml here, too
        toml::from_str(&self.configs[0])
    }
}
