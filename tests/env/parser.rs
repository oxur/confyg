use std::env;
use confyg::env::parser;
use confyg::env::parser::{KV};
// use super::utils;

fn set_vars() {
    env::set_var("MY_PROJ_LOG_LEVEL", "debug");
    env::set_var("MY_PROJ_KEY_1", "value 1");
    env::set_var("MY_PROJ_KEY_2", "value 2");
    env::set_var("MY_PROJ_SECTION_1_KEY_1", "value 3");
    env::set_var("MY_PROJ_SECTION_1_KEY_2", "value 4");
    env::set_var("MY_PROJ_SECTION_2_KEY_1", "value 5");
    env::set_var("MY_PROJ_SECTION_2_KEY_2", "value 6");
    env::set_var("MY_PROJ_SECTION_3_KEY_1", "value 7");
}

#[test]
fn test_scan_env() {
    set_vars();
    let top_level = "my-proj".to_string();
    let s1 = "section-1".to_string();
    let s2 = "section-2".to_string();
    let map = parser::scan(top_level.clone(), vec![s1.clone(), s2.clone()]);
    let keys: Vec<String> = map.keys();
    assert_eq!(keys, vec![top_level.clone(), s1.clone(), s2.clone()]);
    let vals: Vec<Vec<KV>> = map.values();
    assert_eq!(vals[0][2].key, "log_level".to_string());
    assert_eq!(vals[0][2].value, "debug".to_string());
    assert_eq!(vals[1][0].key, "key_1".to_string());
    assert_eq!(vals[1][0].value, "value 3".to_string());
    assert_eq!(vals[2][1].key, "key_2".to_string());
    assert_eq!(vals[2][1].value, "value 6".to_string());
    let sec_data: Vec<String> = map
        .section("section-1")
        .iter()
        .map(|kv| kv.toml())
        .collect();
    assert_eq!(sec_data, vec!["key_1 = 'value 3'", "key_2 = 'value 4'"]);
}
