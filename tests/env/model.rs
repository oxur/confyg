use confyg::env::{KVMap, KV};

#[test]
fn test_kv_new() {
    let kv = KV::new("key1".to_string(), "value1".to_string());
    assert_eq!(kv.key(), "key1");
    assert_eq!(kv.value(), "value1");
}

#[test]
fn test_kv_accessors() {
    let kv = KV::new("test_key".to_string(), "test_value".to_string());
    assert_eq!(kv.key(), "test_key");
    assert_eq!(kv.value(), "test_value");
}

#[test]
fn test_kv_normalise_key() {
    let mut kv = KV::new("MYAPP_DATABASE_HOST".to_string(), "localhost".to_string());
    kv.normalise_key("MYAPP_DATABASE");
    assert_eq!(kv.key(), "host");
}

#[test]
fn test_kv_normalise_key_without_prefix() {
    let mut kv = KV::new("OTHER_KEY".to_string(), "value".to_string());
    kv.normalise_key("MYAPP");
    // Key doesn't have the prefix, so it stays but gets lowercased
    assert_eq!(kv.key(), "other_key");
}

#[test]
fn test_kv_toml() {
    let kv = KV::new("host".to_string(), "localhost".to_string());
    assert_eq!(kv.toml(), "host = 'localhost'");
}

#[test]
fn test_kv_toml_with_special_chars() {
    let kv = KV::new("key".to_string(), "value with spaces".to_string());
    assert_eq!(kv.toml(), "key = 'value with spaces'");
}

#[test]
fn test_kv_ordering() {
    let kv1 = KV::new("aaa".to_string(), "1".to_string());
    let kv2 = KV::new("bbb".to_string(), "2".to_string());

    assert!(kv1 < kv2);
    assert!(kv2 > kv1);
}

#[test]
fn test_kv_equality() {
    let kv1 = KV::new("key".to_string(), "value".to_string());
    let kv2 = KV::new("key".to_string(), "value".to_string());
    let kv3 = KV::new("key".to_string(), "different".to_string());

    assert_eq!(kv1, kv2);
    assert_ne!(kv1, kv3);
}

#[test]
fn test_kv_clone() {
    let kv1 = KV::new("key".to_string(), "value".to_string());
    let kv2 = kv1.clone();

    assert_eq!(kv1, kv2);
}

#[test]
fn test_kv_debug() {
    let kv = KV::new("key".to_string(), "value".to_string());
    let debug_str = format!("{:?}", kv);

    assert!(debug_str.contains("KV"));
    assert!(debug_str.contains("key"));
    assert!(debug_str.contains("value"));
}

#[test]
fn test_kvmap_new() {
    let map = KVMap::new("myapp");
    assert_eq!(map.top_level(), "myapp");
    assert_eq!(map.keys().len(), 0);
}

#[test]
fn test_kvmap_insert_and_keys() {
    let mut map = KVMap::new("myapp");
    let kv1 = KV::new("key1".to_string(), "value1".to_string());
    let kv2 = KV::new("key2".to_string(), "value2".to_string());

    map.insert("section1", vec![kv1]);
    map.insert("section2", vec![kv2]);

    let keys = map.keys();
    assert_eq!(keys.len(), 2);
    // Keys are sorted
    assert!(keys.contains(&"section1".to_string()));
    assert!(keys.contains(&"section2".to_string()));
}

#[test]
fn test_kvmap_values() {
    let mut map = KVMap::new("myapp");
    let kv = KV::new("key".to_string(), "value".to_string());

    map.insert("section", vec![kv]);

    let values = map.values();
    assert_eq!(values.len(), 1);
}

#[test]
fn test_kvmap_section() {
    let mut map = KVMap::new("myapp");
    let kv1 = KV::new("key1".to_string(), "value1".to_string());
    let kv2 = KV::new("key2".to_string(), "value2".to_string());

    map.insert("database", vec![kv2, kv1.clone()]); // Insert in reverse order

    let section = map.section("database");
    assert!(section.is_some());

    let kvs = section.unwrap();
    assert_eq!(kvs.len(), 2);
    // Should be sorted by key
    assert_eq!(kvs[0].key(), "key1");
    assert_eq!(kvs[1].key(), "key2");
}

#[test]
fn test_kvmap_section_not_found() {
    let map = KVMap::new("myapp");
    let section = map.section("nonexistent");
    assert!(section.is_none());
}

#[test]
fn test_kvmap_clone() {
    let mut map1 = KVMap::new("myapp");
    let kv = KV::new("key".to_string(), "value".to_string());
    map1.insert("section", vec![kv]);

    let map2 = map1.clone();
    assert_eq!(map1.top_level(), map2.top_level());
    assert_eq!(map1.keys(), map2.keys());
}

#[test]
fn test_kvmap_debug() {
    let map = KVMap::new("myapp");
    let debug_str = format!("{:?}", map);

    assert!(debug_str.contains("KVMap"));
    assert!(debug_str.contains("myapp"));
}
