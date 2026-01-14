use super::model::{KVMap, KV};
use super::options::Options;
use std::collections::HashMap;
use std::env;

#[derive(Clone, Debug, Default)]
pub struct Scanner {
    opts: Options,
    map: KVMap,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            opts: Options::default(),
            ..Default::default()
        }
    }

    pub fn options(&self) -> &Options {
        &self.opts
    }

    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.opts
    }

    pub fn set_top_level<'a>(&'a mut self, project: &str) -> &'a mut Scanner {
        self.opts.set_top_level(project);
        self
    }

    pub fn add_section<'a>(&'a mut self, section: &str) -> &'a mut Scanner {
        self.opts.add_section(section);
        self
    }

    pub fn scan(&mut self) -> &mut Scanner {
        self.map = scan(self.opts.top_level(), self.opts.sections());
        self
    }

    pub fn toml(&self) -> String {
        self.map.toml()
    }
}

fn env_format(name: &str) -> String {
    name.to_string().to_uppercase().replace('-', "_")
}

pub fn scan(top_level: &str, sections: &[String]) -> KVMap {
    // Define the data structures we're going to use to stuff the env data,
    // to make it toml-like:
    let mut result_map = KVMap::new(top_level);
    let mut seen = Vec::new();
    let mut section_lookup = HashMap::new();
    let mut prefixes = Vec::new();
    let main_prefix = env_format(top_level);
    for section in sections.iter().rev() {
        let prefix = format!("{}_{}", main_prefix, env_format(section));
        section_lookup.insert(prefix.clone(), section.as_str());
        prefixes.push(prefix);
    }
    prefixes.push(main_prefix.clone());
    section_lookup.insert(main_prefix, top_level);
    // Convert the env vars to a vec so we can sort them:
    let mut env_vars = Vec::new();
    for (key, value) in env::vars() {
        env_vars.push(KV::new(key, value));
    }
    env_vars.sort();
    env_vars.reverse();
    // Place env vars with the project prefix into the appropriate
    // section, if one's been passed/defined, otherwise place it in
    // the main/default section:
    for prefix in prefixes.iter() {
        let mut section_vars = Vec::new();
        let section = section_lookup.get(prefix);
        match section {
            None => continue,
            _ => {
                for env_var in &env_vars {
                    if env_var.key().starts_with(prefix) && !seen.contains(env_var) {
                        let mut kv = env_var.clone();
                        kv.normalise_key(prefix);
                        section_vars.push(kv);
                        seen.push(env_var.clone());
                        continue;
                    };
                }
            }
        }
        section_vars.reverse();
        if let Some(section_name) = section {
            result_map.insert(section_name, section_vars);
        }
    }
    result_map
}
