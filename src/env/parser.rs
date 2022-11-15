use std::collections::HashMap;
use std::env;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct KV {
    pub key: String,
    pub value: String,
}

impl KV {
    pub fn new(key: String, value: String) -> Self {
        KV {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn normalise_key(&mut self, prefix: &str) {
        let mut pfx = prefix.clone().to_string();
        pfx.push('_');
        self.key = self.key.clone().replace(&pfx, "").to_ascii_lowercase()
    }
}

#[derive(Clone, Debug, Default)]
pub struct KVMap {
    pub data: HashMap<String, Vec<KV>>
}

impl KVMap {
    pub fn new() -> Self {
        KVMap {
            .. Default::default()
        }
    }

    pub fn insert(&mut self, section: Option<&String>, kvs: Vec<KV>) {
        self.data.insert(section.unwrap().to_string(), kvs);
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.data.clone().into_keys().collect();
        keys.sort();
        keys
    }

    pub fn values(&self) -> Vec<Vec<KV>>  {
        let mut vals: Vec<Vec<KV>> = self.data.clone().into_values().collect();
        vals.sort();
        vals
    }
}

fn env_format(name: &str) -> String {
    name.to_string().to_uppercase().replace("-", "_")
}

pub fn scan(top_level: String, sections: Vec<String>) -> KVMap {
    // Define the data structures we're going to use to stuff the env data,
    // to make it toml-like:
    let mut result_map = KVMap::new();
    let mut seen = Vec::new();
    let mut section_lookup = HashMap::new();
    let mut prefixes = Vec::new();
    let main_prefix = env_format(&top_level);
    for section in sections.iter() {
        let mut prefix = main_prefix.clone();
        prefix.push('_');
        prefix.push_str(&env_format(section));
        section_lookup.insert(prefix.to_string(), section.to_string());
        prefixes.push(prefix);
    }
    prefixes.push(main_prefix.to_string());
    section_lookup.insert(main_prefix.to_string(), top_level.to_string());
    // Convert the env vars to a vec so we can sort them:
    let mut env_vars = Vec::new();
    for(key, value) in env::vars().into_iter() {
        env_vars.push(KV::new(key, value));
    };
    env_vars.sort();
    // Place env vars with the project prefix into the appropriate
    // section, if one's been passed/defined, otherwise place it in
    // the main/default section:
    for prefix in prefixes.iter() {
        let mut section_vars = Vec::new();
        let section = section_lookup.get(prefix);
        match section {
            None => continue,
            _ => {
                for env_var in env_vars.clone().iter() {
                    if env_var.key.starts_with(prefix) && !seen.contains(env_var) {
                        let mut kv = env_var.clone();
                        kv.normalise_key(prefix);
                        section_vars.push(kv);
                        seen.push(env_var.clone());
                        continue;
                    };
                };
            },
        }
        result_map.insert(section, section_vars);
    }
    result_map
}
