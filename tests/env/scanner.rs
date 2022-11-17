use std::env;
use confyg::env::model::KV;
use confyg::env::scanner::{Scanner, scan};
// use super::utils;

fn set_env_vars() {
    env::set_var("MY_PROJ_LOG_LEVEL", "debug");
    env::set_var("MY_PROJ_KEY_1", "value 1");
    env::set_var("MY_PROJ_KEY_2", "value 2");
    env::set_var("MY_PROJ_SECTION_1_KEY_1", "value 3");
    env::set_var("MY_PROJ_SECTION_1_KEY_2", "value 4");
    env::set_var("MY_PROJ_SECTION_2_KEY_1", "value 5");
    env::set_var("MY_PROJ_SECTION_2_KEY_2", "value 6");
    env::set_var("MY_PROJ_SECTION_2_IP", "1.2.3.4");
    env::set_var("MY_PROJ_SECTION_3_KEY_1", "value 7");
}

#[test]
fn test_scan_env() {
    set_env_vars();
    let top_level = "my-proj".to_string();
    let s1 = "section-1".to_string();
    let s2 = "section-2".to_string();
    let map = scan(top_level.clone(), vec![s1.clone(), s2.clone()]);
    let keys: Vec<String> = map.keys();
    assert_eq!(keys, vec![top_level.clone(), s1.clone(), s2.clone()]);
    let vals: Vec<Vec<KV>> = map.values();
    assert_eq!(vals[0][0].key, "ip".to_string());
    assert_eq!(vals[0][0].value, "1.2.3.4".to_string());
    assert_eq!(vals[1][0].key, "key_1".to_string());
    assert_eq!(vals[1][0].value, "value 1".to_string());
    assert_eq!(vals[2][1].key, "key_2".to_string());
    assert_eq!(vals[2][1].value, "value 4".to_string());
}

#[test]
fn test_section_toml() {
    set_env_vars();
    let top_level = "my-proj".to_string();
    let s1 = "section-1".to_string();
    let s2 = "section-2".to_string();
    let map = scan(top_level.clone(), vec![s1.clone(), s2.clone()]);
    assert_eq!(map.section_toml("my-proj"), r#"key_1 = 'value 1'
key_2 = 'value 2'
log_level = 'debug'
section_3_key_1 = 'value 7'"#);
    assert_eq!(map.section_toml("section-1"), r#"[section-1]
key_1 = 'value 3'
key_2 = 'value 4'"#);
}

#[test]
fn test_toml() {
    set_env_vars();
    let top_level = "my-proj".to_string();
    let s1 = "section-1".to_string();
    let s2 = "section-2".to_string();
    let map = scan(top_level.clone(), vec![s1.clone(), s2.clone()]);
    assert_eq!(map.toml(), r#"key_1 = 'value 1'
key_2 = 'value 2'
log_level = 'debug'
section_3_key_1 = 'value 7'

[section-1]
key_1 = 'value 3'
key_2 = 'value 4'

[section-2]
ip = '1.2.3.4'
key_1 = 'value 5'
key_2 = 'value 6'
"#)
}

#[test]
fn test_scanner() {
    set_env_vars();
    let toml = Scanner::new()
        .set_top_level("my-proj")
        .add_section("section-1")
        .add_section("section-2")
        .scan()
        .toml();
    assert_eq!(toml, r#"key_1 = 'value 1'
key_2 = 'value 2'
log_level = 'debug'
section_3_key_1 = 'value 7'

[section-1]
key_1 = 'value 3'
key_2 = 'value 4'

[section-2]
ip = '1.2.3.4'
key_1 = 'value 5'
key_2 = 'value 6'
"#)
}

fn set_env_vars2() {
    env::set_var("CFYG_ENV", "prod");
    env::set_var("CFYG_SERVERS_PLATFORM", "GCP");
    env::set_var("CFYG_SERVERS_DB_HOST", "1.1.2.2");
    env::set_var("CFYG_SERVERS_DB_NAME", "db");
    env::set_var("CFYG_SERVERS_DB_USER", "bob");
    env::set_var("CFYG_SERVERS_DB_MAX_CONNS", "1250");
}

#[test]
fn test_scanner2() {
    set_env_vars2();
    let toml = Scanner::new()
        .set_top_level("cfyg")
        .add_section("servers")
        .add_section("servers_db")
        .scan()
        .toml();
    assert_eq!(toml, r#"env = 'prod'

[servers]
platform = 'GCP'

[servers_db]
host = '1.1.2.2'
max_conns = '1250'
name = 'db'
user = 'bob'
"#)
}
