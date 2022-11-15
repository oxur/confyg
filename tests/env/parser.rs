use std::env;
use confyg::env::parser;
use confyg::env::parser::{EnvVar};
// use super::utils;

#[test]
fn test_collect_env() {
    env::set_var("MY_PROJ_LOG_LEVEL", "debug");
    env::set_var("MY_PROJ_KEY_1", "value 1");
    env::set_var("MY_PROJ_KEY_2", "value 2");
    env::set_var("MY_PROJ_SECTION_1_KEY_1", "value 3");
    env::set_var("MY_PROJ_SECTION_1_KEY_2", "value 4");
    env::set_var("MY_PROJ_SECTION_2_KEY_1", "value 5");
    env::set_var("MY_PROJ_SECTION_2_KEY_2", "value 6");
    env::set_var("MY_PROJ_SECTION_3_KEY_1", "value 7");
    let top_level = "my-proj".to_string();
    let s1 = "section-1".to_string();
    let s2 = "section-2".to_string();
    let map = parser::collect(top_level.clone(), vec![s1.clone(), s2.clone()]);
    let mut keys: Vec<String> = map.clone().into_keys().collect();
    keys.sort();
    assert_eq!(keys, vec![top_level.clone(), s1.clone(), s2.clone()]);
    let mut vals: Vec<Vec<EnvVar>> = map.clone().into_values().collect();
    vals.sort();
    assert_eq!(vals[0][2].key, "MY_PROJ_LOG_LEVEL".to_string());
    assert_eq!(vals[0][2].value, "debug".to_string());
    assert_eq!(vals[1][0].key, "MY_PROJ_SECTION_1_KEY_1".to_string());
    assert_eq!(vals[1][0].value, "value 3".to_string());
    assert_eq!(vals[2][1].key, "MY_PROJ_SECTION_2_KEY_2".to_string());
    assert_eq!(vals[2][1].value, "value 6".to_string());

}
