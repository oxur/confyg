use std::collections::HashMap;
use std::env;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
}

fn env_format(name: &str) -> String {
    name.to_string().to_uppercase().replace("-", "_")
}

fn env_key2toml_key(prefix: &str, key: &str) -> String {
    let mut pfx = prefix.clone().to_string();
    pfx.push('_');
    key.clone().replace(&pfx, "").to_ascii_lowercase()
}

pub fn collect(top_level: String, sections: Vec<String>) -> HashMap<String, Vec<KV>> {
    // Define the data structures we're going to use to stuff the env data,
    // to make it toml-like:
    let mut result_map = HashMap::new();
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
                        kv.key = env_key2toml_key(prefix, &env_var.key);
                        section_vars.push(kv);
                        seen.push(env_var.clone());
                        continue;
                    };
                };
            },
        }
        result_map.insert(section.unwrap().to_string(), section_vars);
    }
    result_map
}
