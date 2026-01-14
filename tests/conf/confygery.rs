use confyg::conf::Options;
use confyg::{env, ConfigError, Confygery};
use serde_derive::{Deserialize, Serialize};
use std::env as std_env;

#[derive(Debug, Deserialize, PartialEq)]
struct SimpleConfig {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ComplexConfig {
    env: String,
    database: DatabaseConfig,
}

#[test]
fn test_confygery_new() {
    let result = Confygery::new();
    assert!(result.is_ok());
    let confyg = result.unwrap();
    assert_eq!(confyg.config_count(), 0);
}

#[test]
fn test_confygery_add_str() {
    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            name = "test"
            version = "1.0.0"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_add_multiple_strings() {
    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_str(r#"name = "test""#)
        .unwrap()
        .add_str(r#"version = "1.0.0""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_override() {
    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            name = "test"
            version = "1.0.0"
        "#,
        )
        .unwrap()
        .add_str(r#"version = "2.0.0""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.version, "2.0.0"); // Later value overrides
}

#[test]
fn test_confygery_with_sections() {
    let config: ComplexConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            env = "dev"
            [database]
            host = "localhost"
            port = 5432
            name = "mydb"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.env, "dev");
    assert_eq!(config.database.host, "localhost");
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.database.name, "mydb");
}

#[test]
fn test_confygery_merge_sections() {
    let config: ComplexConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            env = "dev"
            [database]
            host = "localhost"
            port = 5432
            name = "mydb"
        "#,
        )
        .unwrap()
        .add_str(
            r#"
            [database]
            host = "production-db"
            port = 3306
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.env, "dev");
    assert_eq!(config.database.host, "production-db"); // Overridden
    assert_eq!(config.database.port, 3306); // Overridden
    assert_eq!(config.database.name, "mydb"); // Not overridden, kept from first
}

#[test]
fn test_confygery_add_struct() {
    #[derive(Serialize)]
    struct Defaults {
        name: String,
        version: String,
    }

    let defaults = Defaults {
        name: "myapp".to_string(),
        version: "1.0.0".to_string(),
    };

    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_struct(&defaults)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "myapp");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_add_env() {
    // Set environment variables
    std_env::set_var("TEST_CONFYG_NAME", "envapp");
    std_env::set_var("TEST_CONFYG_VERSION", "2.0.0");

    let mut env_opts = env::Options::with_top_level("test_confyg");
    env_opts.add_section("ignored"); // Won't match our SimpleConfig

    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_env(env_opts)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "envapp");
    assert_eq!(config.version, "2.0.0");

    // Cleanup
    std_env::remove_var("TEST_CONFYG_NAME");
    std_env::remove_var("TEST_CONFYG_VERSION");
}

#[test]
fn test_confygery_add_file() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct ServersDB {
        host: String,
        name: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct FileConfig {
        env: String,
        servers_db: ServersDB,
    }

    let mut opts = Options::default();
    opts.add_path("examples/confs");

    let config: FileConfig = Confygery::new()
        .unwrap()
        .with_opts(opts)
        .unwrap()
        .add_file("common-under.toml")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.servers_db.host, "1.1.1.1");
    assert_eq!(config.servers_db.name, "db");
}

#[test]
fn test_confygery_add_file_not_found() {
    // File not found should NOT error (silent ignore)
    let result = Confygery::new()
        .unwrap()
        .add_file("nonexistent.toml")
        .unwrap()
        .add_str(r#"name = "test""#)
        .unwrap()
        .add_str(r#"version = "1.0.0""#)
        .unwrap()
        .build::<SimpleConfig>();

    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.name, "test");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_add_file_merge() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct ServersDB {
        host: String,
        name: String,
        user: String,
        max_conns: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct FileConfig {
        env: String,
        servers_db: ServersDB,
    }

    let mut opts = Options::default();
    opts.add_path("examples/confs");

    let config: FileConfig = Confygery::new()
        .unwrap()
        .with_opts(opts)
        .unwrap()
        .add_file("common-under.toml")
        .unwrap()
        .add_file("testing-under.toml")
        .unwrap()
        .build()
        .unwrap();

    // testing-under.toml should override common-under.toml
    assert_eq!(config.env, "testing");
    assert_eq!(config.servers_db.host, "2.3.4.5"); // Overridden
    assert_eq!(config.servers_db.name, "testing-db"); // Overridden
    assert_eq!(config.servers_db.user, "carol"); // Overridden
    assert_eq!(config.servers_db.max_conns, "500"); // NOT overridden (empty strings don't override)
}

#[test]
fn test_confygery_no_configs_error() {
    let result = Confygery::new().unwrap().build::<SimpleConfig>();

    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::NoConfigs => (),
        e => panic!("Expected NoConfigs error, got: {:?}", e),
    }
}

#[test]
fn test_confygery_invalid_toml() {
    let result = Confygery::new()
        .unwrap()
        .add_str("this is not valid TOML {{{}}")
        .unwrap()
        .build::<SimpleConfig>();

    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::TomlParse { .. } => (),
        e => panic!("Expected TomlParse error, got: {:?}", e),
    }
}

#[test]
fn test_confygery_options_accessor() {
    let confyg = Confygery::new().unwrap();
    let opts = confyg.options();
    assert_eq!(opts.paths().len(), 0);
}

#[test]
fn test_confygery_options_mut_accessor() {
    let mut confyg = Confygery::new().unwrap();
    confyg.options_mut().add_path("./test");
    assert_eq!(confyg.options().paths().len(), 1);
    assert_eq!(confyg.options().paths()[0], "./test");
}

#[test]
fn test_confygery_config_count() {
    let mut confyg = Confygery::new().unwrap();
    assert_eq!(confyg.config_count(), 0);

    confyg.add_str(r#"name = "test""#).unwrap();
    assert_eq!(confyg.config_count(), 1);

    confyg.add_str(r#"version = "1.0""#).unwrap();
    assert_eq!(confyg.config_count(), 2);
}

#[test]
fn test_confygery_with_opts() {
    let mut opts = Options::default();
    opts.add_path("./config").set_project("myapp");

    let mut confyg = Confygery::new().unwrap();
    confyg.with_opts(opts).unwrap();

    assert_eq!(confyg.options().paths().len(), 1);
    assert_eq!(confyg.options().paths()[0], "./config");
    assert_eq!(confyg.options().project(), "myapp");
}

#[test]
fn test_confygery_default() {
    let confyg = Confygery::default();
    assert_eq!(confyg.config_count(), 0);
    assert_eq!(confyg.options().paths().len(), 0);
}

#[test]
fn test_confygery_clone() {
    let mut confyg1 = Confygery::new().unwrap();
    confyg1.add_str(r#"name = "test""#).unwrap();
    confyg1.options_mut().add_path("./config");

    let confyg2 = confyg1.clone();
    assert_eq!(confyg2.config_count(), 1);
    assert_eq!(confyg2.options().paths().len(), 1);
}

#[test]
fn test_confygery_debug() {
    let confyg = Confygery::new().unwrap();
    let debug_str = format!("{:?}", confyg);
    assert!(debug_str.contains("Confygery"));
}

#[test]
fn test_confygery_add_struct_override() {
    #[derive(Serialize)]
    struct Defaults {
        name: String,
        version: String,
    }

    #[derive(Serialize)]
    struct Overrides {
        version: String,
    }

    let defaults = Defaults {
        name: "myapp".to_string(),
        version: "1.0.0".to_string(),
    };

    let overrides = Overrides {
        version: "2.0.0".to_string(),
    };

    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_struct(&defaults)
        .unwrap()
        .add_struct(&overrides)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "myapp");
    assert_eq!(config.version, "2.0.0"); // Overridden
}

#[test]
fn test_confygery_mixed_sources() {
    // Test mixing all source types: str, struct, file, env
    #[derive(Serialize)]
    struct Defaults {
        name: String,
    }

    let defaults = Defaults {
        name: "base".to_string(),
    };

    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_struct(&defaults)
        .unwrap()
        .add_str(r#"version = "1.0.0""#)
        .unwrap()
        .add_file("nonexistent.toml")
        .unwrap() // Should be ignored
        .build()
        .unwrap();

    assert_eq!(config.name, "base");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_build_type_mismatch() {
    // Build with wrong type should error during deserialization
    #[derive(Debug, Deserialize, PartialEq)]
    struct WrongConfig {
        name: String,
        count: u32, // This field doesn't exist in our data
    }

    let result = Confygery::new()
        .unwrap()
        .add_str(r#"name = "test""#)
        .unwrap()
        .build::<WrongConfig>();

    assert!(result.is_err());
}

#[test]
fn test_confygery_multiple_merges() {
    // Test merging multiple configurations with complex override behavior
    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_str(r#"name = "first""#)
        .unwrap()
        .add_str(r#"version = "1.0.0""#)
        .unwrap()
        .add_str(r#"name = "second""#)
        .unwrap()
        .add_str(r#"version = "2.0.0""#)
        .unwrap()
        .add_str(r#"name = "final""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "final");
    assert_eq!(config.version, "2.0.0");
}

#[test]
fn test_confygery_empty_toml() {
    // Adding empty TOML string should still be counted
    let mut confyg = Confygery::new().unwrap();
    confyg.add_str("").unwrap();
    confyg.add_str(r#"name = "test""#).unwrap();
    confyg.add_str(r#"version = "1.0.0""#).unwrap();

    assert_eq!(confyg.config_count(), 3);

    let config: SimpleConfig = confyg.build().unwrap();
    assert_eq!(config.name, "test");
}

#[test]
fn test_confygery_with_opts_and_sources() {
    let mut opts = Options::default();
    opts.add_path("examples/confs");

    #[derive(Debug, Deserialize, PartialEq)]
    struct ServersDB {
        host: String,
        name: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct FileConfig {
        env: String,
        servers_db: ServersDB,
    }

    let config: FileConfig = Confygery::new()
        .unwrap()
        .with_opts(opts)
        .unwrap()
        .add_file("common-under.toml")
        .unwrap()
        .add_str(r#"env = "custom""#)
        .unwrap()
        .build()
        .unwrap();

    // String should override the file added first
    assert_eq!(config.env, "custom");
    assert_eq!(config.servers_db.host, "1.1.1.1");
}

#[test]
fn test_confygery_add_struct_with_complex_types() {
    #[derive(Serialize)]
    struct ComplexStruct {
        name: String,
        count: u32,
        enabled: bool,
        tags: Vec<String>,
    }

    let complex = ComplexStruct {
        name: "complex".to_string(),
        count: 42,
        enabled: true,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };

    #[derive(Debug, Deserialize, PartialEq)]
    struct ComplexConfig {
        name: String,
        count: u32,
        enabled: bool,
        tags: Vec<String>,
    }

    let config: ComplexConfig = Confygery::new()
        .unwrap()
        .add_struct(&complex)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "complex");
    assert_eq!(config.count, 42);
    assert_eq!(config.enabled, true);
    assert_eq!(config.tags, vec!["tag1", "tag2"]);
}

#[test]
fn test_confygery_add_env_with_multiple_sections() {
    // Set environment variables for multiple sections
    std_env::set_var("TEST_CONFYG2_APP_NAME", "myapp");
    std_env::set_var("TEST_CONFYG2_APP_VERSION", "1.0.0");
    std_env::set_var("TEST_CONFYG2_DB_HOST", "localhost");
    std_env::set_var("TEST_CONFYG2_DB_PORT", "5432");

    let mut env_opts = env::Options::with_top_level("test_confyg2");
    env_opts.add_section("app").add_section("db");

    #[derive(Debug, Deserialize, PartialEq)]
    struct App {
        name: String,
        version: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Db {
        host: String,
        port: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct EnvConfig {
        app: App,
        db: Db,
    }

    let config: EnvConfig = Confygery::new()
        .unwrap()
        .add_env(env_opts)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.app.name, "myapp");
    assert_eq!(config.app.version, "1.0.0");
    assert_eq!(config.db.host, "localhost");
    assert_eq!(config.db.port, "5432");

    // Cleanup
    std_env::remove_var("TEST_CONFYG2_APP_NAME");
    std_env::remove_var("TEST_CONFYG2_APP_VERSION");
    std_env::remove_var("TEST_CONFYG2_DB_HOST");
    std_env::remove_var("TEST_CONFYG2_DB_PORT");
}

#[test]
fn test_confygery_chained_builder() {
    // Test fully chained builder pattern
    let mut opts = Options::default();
    opts.add_path("examples/confs");

    #[derive(Debug, Deserialize, PartialEq)]
    struct ServersDB {
        host: String,
        name: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct ChainConfig {
        env: String,
        servers_db: ServersDB,
    }

    let mut confyg = Confygery::new().unwrap();
    let config: ChainConfig = confyg
        .with_opts(opts)
        .unwrap()
        .add_file("common-under.toml")
        .unwrap()
        .add_str(r#"env = "override""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.env, "override");
}

#[test]
fn test_confygery_merge_with_single_config() {
    // Edge case: merging with only one config
    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            name = "single"
            version = "1.0.0"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "single");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_add_str_maintains_order() {
    // Ensure that configs maintain insertion order during merge
    let mut confyg = Confygery::new().unwrap();

    confyg.add_str(r#"name = "first""#).unwrap();
    confyg.add_str(r#"name = "second""#).unwrap();
    confyg.add_str(r#"name = "third""#).unwrap();
    confyg.add_str(r#"version = "1.0.0""#).unwrap();

    let config: SimpleConfig = confyg.build().unwrap();

    // Last value for 'name' should win
    assert_eq!(config.name, "third");
    assert_eq!(config.version, "1.0.0");
}

#[test]
fn test_confygery_options_mutation() {
    // Test that options can be mutated after creation
    let mut confyg = Confygery::new().unwrap();

    assert_eq!(confyg.options().paths().len(), 0);

    confyg.options_mut().add_path("./config");
    confyg.options_mut().add_path("./settings");

    assert_eq!(confyg.options().paths().len(), 2);
    assert_eq!(confyg.options().paths()[0], "./config");
    assert_eq!(confyg.options().paths()[1], "./settings");
}

#[test]
fn test_confygery_add_file_with_invalid_path() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Create a directory with a file we can't read
    let test_dir = "/tmp/confyg_test_permissions";
    let test_file = format!("{}/unreadable.toml", test_dir);

    // Cleanup if exists
    let _ = fs::remove_dir_all(test_dir);

    fs::create_dir_all(test_dir).unwrap();
    fs::write(&test_file, "key = 'value'").unwrap();

    // Make file unreadable
    let metadata = fs::metadata(&test_file).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&test_file, permissions).unwrap();

    let mut opts = Options::default();
    opts.add_path(test_dir);

    let mut confyg = Confygery::new().unwrap();
    confyg.with_opts(opts).unwrap();
    let result = confyg.add_file("unreadable.toml");

    // Should get a FileRead error
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::FileRead { .. } => (),
        e => panic!("Expected FileRead error, got: {:?}", e),
    }

    // Cleanup - restore permissions first
    let metadata = fs::metadata(&test_file).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o644);
    let _ = fs::set_permissions(&test_file, permissions);
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_confygery_merge_all_with_loop() {
    // Test merge_all with multiple iterations through the loop
    let config: SimpleConfig = Confygery::new()
        .unwrap()
        .add_str(r#"name = "base""#)
        .unwrap()
        .add_str(r#"version = "1.0.0""#)
        .unwrap()
        .add_str(r#"name = "middle""#)
        .unwrap()
        .add_str(r#"version = "2.0.0""#)
        .unwrap()
        .add_str(r#"name = "final""#)
        .unwrap()
        .add_str(r#"version = "3.0.0""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.name, "final");
    assert_eq!(config.version, "3.0.0");
}

#[test]
fn test_confygery_toml_serialize_error() {
    // Test struct that might cause serialization issues
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct NestedStruct {
        data: HashMap<String, String>,
    }

    let mut data = HashMap::new();
    data.insert("key1".to_string(), "value1".to_string());
    data.insert("key2".to_string(), "value2".to_string());

    let nested = NestedStruct { data };

    let mut confyg = Confygery::new().unwrap();
    let result = confyg.add_struct(&nested);

    // Should succeed
    assert!(result.is_ok());
}

#[test]
fn test_confygery_table_merge_in_build() {
    // Test the Table wrapping in merge_all (line 319)
    #[derive(Debug, Deserialize, PartialEq)]
    struct Section {
        key: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TableConfig {
        section: Section,
    }

    let config: TableConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            [section]
            key = "first"
        "#,
        )
        .unwrap()
        .add_str(
            r#"
            [section]
            key = "second"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.section.key, "second");
}
