use confyg::Confygery;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct MergeConfig {
    key1: String,
    key2: Option<String>,
    key3: Option<String>,
    nested: Option<NestedConfig>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NestedConfig {
    value1: String,
    value2: Option<String>,
}

#[test]
fn test_merge_simple_override() {
    let config: MergeConfig = Confygery::new()
        .unwrap()
        .add_str(r#"key1 = "original""#)
        .unwrap()
        .add_str(r#"key1 = "overridden""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.key1, "overridden");
}

#[test]
fn test_merge_add_new_keys() {
    let config: MergeConfig = Confygery::new()
        .unwrap()
        .add_str(r#"key1 = "first""#)
        .unwrap()
        .add_str(r#"key2 = "second""#)
        .unwrap()
        .add_str(r#"key3 = "third""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.key1, "first");
    assert_eq!(config.key2, Some("second".to_string()));
    assert_eq!(config.key3, Some("third".to_string()));
}

#[test]
fn test_merge_nested_sections() {
    let config: MergeConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            key1 = "main"
            [nested]
            value1 = "original"
        "#,
        )
        .unwrap()
        .add_str(
            r#"
            [nested]
            value2 = "added"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.key1, "main");
    assert!(config.nested.is_some());
    let nested = config.nested.unwrap();
    assert_eq!(nested.value1, "original");
    assert_eq!(nested.value2, Some("added".to_string()));
}

#[test]
fn test_merge_nested_override() {
    let config: MergeConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            key1 = "main"
            [nested]
            value1 = "original"
            value2 = "first"
        "#,
        )
        .unwrap()
        .add_str(
            r#"
            [nested]
            value1 = "overridden"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.key1, "main");
    assert!(config.nested.is_some());
    let nested = config.nested.unwrap();
    assert_eq!(nested.value1, "overridden");
    assert_eq!(nested.value2, Some("first".to_string()));
}

#[test]
fn test_merge_priority_order() {
    // Later sources should override earlier ones
    let config: MergeConfig = Confygery::new()
        .unwrap()
        .add_str(r#"key1 = "first""#)
        .unwrap()
        .add_str(r#"key1 = "second""#)
        .unwrap()
        .add_str(r#"key1 = "third""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.key1, "third");
}

#[test]
fn test_merge_empty_string_does_not_override() {
    // Empty strings do NOT override by design
    #[derive(Debug, Deserialize, PartialEq)]
    struct EmptyConfig {
        value: String,
    }

    let config: EmptyConfig = Confygery::new()
        .unwrap()
        .add_str(r#"value = "something""#)
        .unwrap()
        .add_str(r#"value = """#) // Empty string - should NOT override
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.value, "something"); // Original value preserved
}

#[test]
fn test_merge_non_empty_string_overrides() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct StringConfig {
        value: String,
    }

    let config: StringConfig = Confygery::new()
        .unwrap()
        .add_str(r#"value = "original""#)
        .unwrap()
        .add_str(r#"value = "overridden""#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.value, "overridden");
}

#[test]
fn test_merge_numbers() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct NumberConfig {
        count: i32,
        value: f64,
    }

    let config: NumberConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            count = 10
            value = 1.5
        "#,
        )
        .unwrap()
        .add_str(
            r#"
            count = 20
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.count, 20);
    assert_eq!(config.value, 1.5);
}

#[test]
fn test_merge_boolean() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct BoolConfig {
        enabled: bool,
        debug: Option<bool>,
    }

    let config: BoolConfig = Confygery::new()
        .unwrap()
        .add_str(r#"enabled = false"#)
        .unwrap()
        .add_str(
            r#"
            enabled = true
            debug = false
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.enabled, true);
    assert_eq!(config.debug, Some(false));
}

#[test]
fn test_merge_arrays() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct ArrayConfig {
        items: Vec<String>,
    }

    let config: ArrayConfig = Confygery::new()
        .unwrap()
        .add_str(r#"items = ["a", "b"]"#)
        .unwrap()
        .add_str(r#"items = ["c", "d", "e"]"#) // Should replace, not append
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.items, vec!["c", "d", "e"]);
}

#[test]
fn test_merge_integer_zero_does_not_override() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct IntConfig {
        value: i64,
    }

    let config: IntConfig = Confygery::new()
        .unwrap()
        .add_str(r#"value = 42"#)
        .unwrap()
        .add_str(r#"value = 0"#) // Zero should NOT override
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.value, 42); // Original value preserved
}

#[test]
fn test_merge_float_zero_does_not_override() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct FloatConfig {
        value: f64,
    }

    let config: FloatConfig = Confygery::new()
        .unwrap()
        .add_str(r#"value = 3.14"#)
        .unwrap()
        .add_str(r#"value = 0.0"#) // Zero should NOT override
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.value, 3.14); // Original value preserved
}

#[test]
fn test_merge_boolean_false_does_not_override() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct BoolConfig {
        enabled: bool,
    }

    let config: BoolConfig = Confygery::new()
        .unwrap()
        .add_str(r#"enabled = true"#)
        .unwrap()
        .add_str(r#"enabled = false"#) // false should NOT override true
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.enabled, true); // Original value preserved
}

#[test]
fn test_merge_datetime() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct DateConfig {
        timestamp: toml::value::Datetime,
    }

    let config: DateConfig = Confygery::new()
        .unwrap()
        .add_str(r#"timestamp = 2024-01-01T00:00:00Z"#)
        .unwrap()
        .add_str(r#"timestamp = 2024-12-31T23:59:59Z"#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.timestamp.to_string(), "2024-12-31T23:59:59Z");
}

#[test]
fn test_merge_array_element_by_element() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct ArrayConfig {
        items: Vec<String>,
    }

    // When arrays have same length, merge element-by-element
    let config: ArrayConfig = Confygery::new()
        .unwrap()
        .add_str(r#"items = ["a", "b", "c"]"#)
        .unwrap()
        .add_str(r#"items = ["", "B", ""]"#) // Empty strings won't override
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.items, vec!["a", "B", "c"]);
}

#[test]
fn test_merge_array_type_mismatch() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct MismatchConfig {
        value: Vec<String>,
    }

    // When merged value is not an array, replace it entirely
    let config: MismatchConfig = Confygery::new()
        .unwrap()
        .add_str(r#"value = ["a", "b"]"#)
        .unwrap()
        .add_str(r#"value = ["c", "d", "e"]"#)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.value, vec!["c", "d", "e"]);
}

#[test]
fn test_merge_table_type_mismatch() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TableConfig {
        section: Section,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Section {
        key: String,
    }

    // Tables should merge properly
    let config: TableConfig = Confygery::new()
        .unwrap()
        .add_str(
            r#"
            [section]
            key = "original"
        "#,
        )
        .unwrap()
        .add_str(
            r#"
            [section]
            key = "updated"
        "#,
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.section.key, "updated");
}
