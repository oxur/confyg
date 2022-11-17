use std::collections::HashMap;

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

    pub fn toml(&self) -> String {
        format!("{} = '{}'", self.key, self.value)
    }
}

#[derive(Clone, Debug, Default)]
pub struct KVMap {
    pub top_level: String,
    pub data: HashMap<String, Vec<KV>>,
}

impl KVMap {
    pub fn new(top_level: &str) -> Self {
        KVMap {
            top_level: top_level.to_string(),

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

    pub fn section(&self, name: &str) -> Vec<KV> {
        let mut s = self.data.get(name).unwrap().to_vec();
        s.sort_by(|a, b| a.key.cmp(&b.key));
        s
    }

    pub fn section_toml(&self, name: &str) -> String {
        let mut toml = String::new();
        if name != self.top_level {
            toml.push_str(&format!("[{}]\n", name))
        }
        let mut body = self.section(name)
            .iter()
            .map(|kv| kv.toml())
            .collect::<Vec<String>>();
        body.sort();
        toml.push_str(&body.join("\n"));
        toml
    }

    pub fn toml(&self) -> String {
        let mut toml = self.section_toml(&self.top_level);
        toml.push_str("\n");
        for key in self.keys() {
            if key != self.top_level {
                toml.push_str("\n");
                toml.push_str(&self.section_toml(&key));
                toml.push_str("\n");
            }
        }
        toml
    }
}
