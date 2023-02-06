use toml;

pub fn merge(merged: &mut toml::Value, value: &toml::Value) {
    match value {
        toml::Value::String(x) => {
            if x.is_empty() {
                *merged = value.clone();
            }
        }
        toml::Value::Integer(x) => {
            if x != &0_i64 {
                *merged = value.clone();
            }
        }
        toml::Value::Float(x) => {
            if x != &0.0_f64 {
                *merged = value.clone();
            }
        }
        toml::Value::Boolean(x) => {
            if x != &false {
                *merged = value.clone();
            }
        }
        toml::Value::Datetime(_) => {
            *merged = value.clone();
        }
        toml::Value::Array(x) => match merged {
            toml::Value::Array(merged) => {
                for (k, v) in x.iter().enumerate() {
                    match merged.get_mut(k) {
                        Some(x) => merge(x, v),
                        None => {
                            merged.insert(k, v.clone());
                        }
                    }
                }
            }
            _ => *merged = value.clone(),
        },
        toml::Value::Table(x) => match merged {
            toml::Value::Table(merged) => {
                for (k, v) in x.iter() {
                    match merged.get_mut(k) {
                        Some(x) => merge(x, v),
                        None => {
                            let _ = merged.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
            _ => *merged = value.clone(),
        },
    }
}
