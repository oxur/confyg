use confyg::env::Options;

#[test]
fn test_options_new() {
    let opts = Options::new();
    assert_eq!(opts.top_level(), "");
    assert_eq!(opts.sections().len(), 0);
}

#[test]
fn test_options_default() {
    let opts = Options::default();
    assert_eq!(opts.top_level(), "");
    assert_eq!(opts.sections().len(), 0);
}

#[test]
fn test_options_with_top_level() {
    let opts = Options::with_top_level("myapp");
    assert_eq!(opts.top_level(), "myapp");
    assert_eq!(opts.sections().len(), 0);
}

#[test]
fn test_options_set_top_level() {
    let mut opts = Options::default();
    assert_eq!(opts.top_level(), "");

    opts.set_top_level("myapp");
    assert_eq!(opts.top_level(), "myapp");
}

#[test]
fn test_options_set_top_level_replaces() {
    let mut opts = Options::with_top_level("oldapp");
    assert_eq!(opts.top_level(), "oldapp");

    opts.set_top_level("newapp");
    assert_eq!(opts.top_level(), "newapp");
}

#[test]
fn test_options_add_section() {
    let mut opts = Options::with_top_level("myapp");
    opts.add_section("database");

    assert_eq!(opts.sections().len(), 1);
    assert_eq!(opts.sections()[0], "database");
}

#[test]
fn test_options_add_multiple_sections() {
    let mut opts = Options::with_top_level("myapp");
    opts.add_section("database").add_section("cache");

    assert_eq!(opts.sections().len(), 2);
    assert_eq!(opts.sections()[0], "database");
    assert_eq!(opts.sections()[1], "cache");
}

#[test]
fn test_options_set_sections() {
    let mut opts = Options::with_top_level("myapp");
    opts.set_sections(vec![
        "database".to_string(),
        "cache".to_string(),
        "queue".to_string(),
    ]);

    assert_eq!(opts.sections().len(), 3);
    assert_eq!(opts.sections()[0], "database");
    assert_eq!(opts.sections()[1], "cache");
    assert_eq!(opts.sections()[2], "queue");
}

#[test]
fn test_options_set_sections_replaces() {
    let mut opts = Options::with_top_level("myapp");
    opts.add_section("old");
    assert_eq!(opts.sections().len(), 1);

    opts.set_sections(vec!["new1".to_string(), "new2".to_string()]);
    assert_eq!(opts.sections().len(), 2);
    assert_eq!(opts.sections()[0], "new1");
    assert_eq!(opts.sections()[1], "new2");
}

#[test]
fn test_options_builder_pattern() {
    let mut opts = Options::with_top_level("myapp");
    opts.add_section("database")
        .add_section("cache")
        .set_top_level("newapp");

    assert_eq!(opts.top_level(), "newapp");
    assert_eq!(opts.sections().len(), 2);
}

#[test]
fn test_options_equality() {
    let opts1 = Options::with_top_level("myapp");
    let opts2 = Options::with_top_level("myapp");
    let opts3 = Options::with_top_level("other");

    assert_eq!(opts1, opts2);
    assert_ne!(opts1, opts3);
}

#[test]
fn test_options_equality_with_sections() {
    let mut opts1 = Options::with_top_level("myapp");
    opts1.add_section("db");

    let mut opts2 = Options::with_top_level("myapp");
    opts2.add_section("db");

    let mut opts3 = Options::with_top_level("myapp");
    opts3.add_section("cache");

    assert_eq!(opts1, opts2);
    assert_ne!(opts1, opts3);
}

#[test]
fn test_options_clone() {
    let mut opts1 = Options::with_top_level("myapp");
    opts1.add_section("database");

    let opts2 = opts1.clone();
    assert_eq!(opts1, opts2);
    assert_eq!(opts2.top_level(), "myapp");
    assert_eq!(opts2.sections().len(), 1);
}

#[test]
fn test_options_debug() {
    let mut opts = Options::with_top_level("myapp");
    opts.add_section("db");

    let debug_str = format!("{:?}", opts);
    assert!(debug_str.contains("Options"));
    assert!(debug_str.contains("myapp"));
    assert!(debug_str.contains("db"));
}
