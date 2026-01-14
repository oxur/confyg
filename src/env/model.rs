use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct KV {
    key: String,
    value: String,
}

impl KV {
    pub fn new(key: String, value: String) -> Self {
        KV { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn normalise_key(&mut self, prefix: &str) {
        // Use strip_prefix to avoid multiple allocations
        let prefix_with_sep = format!("{prefix}_");
        self.key = self
            .key
            .strip_prefix(&prefix_with_sep)
            .unwrap_or(&self.key)
            .to_ascii_lowercase();
    }

    pub fn toml(&self) -> String {
        format!("{} = '{}'", self.key, self.value)
    }
}

#[derive(Clone, Debug, Default)]
pub struct KVMap {
    top_level: String,
    data: HashMap<String, Vec<KV>>,
}

impl KVMap {
    pub fn new(top_level: &str) -> Self {
        KVMap {
            top_level: top_level.to_string(),
            ..Default::default()
        }
    }

    pub fn top_level(&self) -> &str {
        &self.top_level
    }

    pub fn insert(&mut self, section: &str, kvs: Vec<KV>) {
        self.data.insert(section.to_string(), kvs);
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.data.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn values(&self) -> Vec<Vec<KV>> {
        let mut vals: Vec<Vec<KV>> = self.data.values().cloned().collect();
        vals.sort();
        vals
    }

    pub fn section(&self, name: &str) -> Option<Vec<KV>> {
        self.data.get(name).map(|s| {
            let mut result = s.clone();
            result.sort_by(|a, b| a.key.cmp(&b.key));
            result
        })
    }

    pub fn section_toml(&self, name: &str) -> String {
        let mut toml = String::new();
        if name != self.top_level {
            toml.push_str(&format!("[{name}]\n"))
        }
        if let Some(section) = self.section(name) {
            let mut body: Vec<String> = section.iter().map(|kv| kv.toml()).collect();
            body.sort();
            toml.push_str(&body.join("\n"));
        }
        toml
    }

    pub fn toml(&self) -> String {
        let mut toml = self.section_toml(&self.top_level);
        toml.push('\n');
        for key in self.keys() {
            if key != self.top_level {
                toml.push('\n');
                toml.push_str(&self.section_toml(&key));
                toml.push('\n');
            }
        }
        toml
    }
}
