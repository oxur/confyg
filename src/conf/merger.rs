use toml;

/// Recursively merges TOML values.
///
/// Merge strategy:
/// - Scalars: Replace if non-empty/non-zero/true
/// - Arrays: Merge element-by-element
/// - Tables: Recursively merge keys
pub fn merge(merged: &mut toml::Value, value: &toml::Value) {
    match value {
        toml::Value::String(x) => {
            // Only replace if the new string is non-empty
            if !x.is_empty() {
                *merged = value.clone();
            }
        }
        toml::Value::Integer(x) => {
            if *x != 0 {
                *merged = value.clone();
            }
        }
        toml::Value::Float(x) => {
            if *x != 0.0 {
                *merged = value.clone();
            }
        }
        toml::Value::Boolean(x) => {
            if *x {
                *merged = value.clone();
            }
        }
        toml::Value::Datetime(_) => {
            *merged = value.clone();
        }
        toml::Value::Array(x) => match merged {
            toml::Value::Array(merged_arr) => {
                for (k, v) in x.iter().enumerate() {
                    match merged_arr.get_mut(k) {
                        Some(existing) => merge(existing, v),
                        None => {
                            // Clone needed: inserting into Vec
                            merged_arr.push(v.clone());
                        }
                    }
                }
            }
            _ => *merged = value.clone(),
        },
        toml::Value::Table(x) => match merged {
            toml::Value::Table(merged_table) => {
                for (k, v) in x.iter() {
                    match merged_table.get_mut(k) {
                        Some(existing) => merge(existing, v),
                        None => {
                            // Clones needed: HashMap takes ownership
                            merged_table.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
            _ => *merged = value.clone(),
        },
    }
}
