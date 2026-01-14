use confyg::searchpath::Options;

#[test]
fn test_options_new() {
    let opts = Options::new();
    assert_eq!(opts.paths().len(), 0);
}

#[test]
fn test_options_default() {
    let opts = Options::default();
    assert_eq!(opts.paths().len(), 0);
}

#[test]
fn test_options_add_path() {
    let mut opts = Options::new();
    opts.add_path("./config");

    assert_eq!(opts.paths().len(), 1);
    assert_eq!(opts.paths()[0], "./config");
}

#[test]
fn test_options_add_multiple_paths() {
    let mut opts = Options::new();
    opts.add_path("./config").add_path("/etc/app");

    assert_eq!(opts.paths().len(), 2);
    assert_eq!(opts.paths()[0], "./config");
    assert_eq!(opts.paths()[1], "/etc/app");
}

#[test]
fn test_options_set_paths() {
    let mut opts = Options::new();
    opts.set_paths(vec![
        "./config".to_string(),
        "/etc/app".to_string(),
        "/usr/local/etc/app".to_string(),
    ]);

    assert_eq!(opts.paths().len(), 3);
    assert_eq!(opts.paths()[0], "./config");
    assert_eq!(opts.paths()[1], "/etc/app");
    assert_eq!(opts.paths()[2], "/usr/local/etc/app");
}

#[test]
fn test_options_set_paths_replaces() {
    let mut opts = Options::new();
    opts.add_path("./old");
    assert_eq!(opts.paths().len(), 1);

    opts.set_paths(vec!["./new1".to_string(), "./new2".to_string()]);
    assert_eq!(opts.paths().len(), 2);
    assert_eq!(opts.paths()[0], "./new1");
    assert_eq!(opts.paths()[1], "./new2");
}

#[test]
fn test_options_builder_pattern() {
    let mut opts = Options::new();
    opts.add_path("./config")
        .add_path("/etc/app")
        .add_path("/usr/local/etc/app");

    assert_eq!(opts.paths().len(), 3);
}

#[test]
fn test_options_equality() {
    let mut opts1 = Options::new();
    opts1.add_path("./config");

    let mut opts2 = Options::new();
    opts2.add_path("./config");

    let mut opts3 = Options::new();
    opts3.add_path("./other");

    assert_eq!(opts1, opts2);
    assert_ne!(opts1, opts3);
}

#[test]
fn test_options_clone() {
    let mut opts1 = Options::new();
    opts1.add_path("./config");

    let opts2 = opts1.clone();
    assert_eq!(opts1, opts2);
    assert_eq!(opts2.paths().len(), 1);
    assert_eq!(opts2.paths()[0], "./config");
}

#[test]
fn test_options_debug() {
    let mut opts = Options::new();
    opts.add_path("./config");

    let debug_str = format!("{:?}", opts);
    assert!(debug_str.contains("Options"));
    assert!(debug_str.contains("config"));
}
