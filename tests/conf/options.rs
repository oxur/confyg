use confyg::conf::Options;

#[test]
fn test_options_new() {
    let opts = Options::new();
    assert_eq!(opts.paths().len(), 0);
    assert_eq!(opts.project(), "");
}

#[test]
fn test_options_default() {
    let opts = Options::default();
    assert_eq!(opts.paths().len(), 0);
    assert_eq!(opts.project(), "");
}

#[test]
fn test_options_with_project() {
    let opts = Options::with_project("myapp");
    assert_eq!(opts.project(), "myapp");
    assert_eq!(opts.paths().len(), 0);
}

#[test]
fn test_options_add_path() {
    let mut opts = Options::default();
    opts.add_path("./config");
    assert_eq!(opts.paths().len(), 1);
    assert_eq!(opts.paths()[0], "./config");
}

#[test]
fn test_options_add_multiple_paths() {
    let mut opts = Options::default();
    opts.add_path("./config").add_path("/etc/myapp");
    assert_eq!(opts.paths().len(), 2);
    assert_eq!(opts.paths()[0], "./config");
    assert_eq!(opts.paths()[1], "/etc/myapp");
}

#[test]
fn test_options_set_paths() {
    let mut opts = Options::default();
    opts.set_paths(vec![
        "./config".to_string(),
        "/etc/myapp".to_string(),
        "/usr/local/etc/myapp".to_string(),
    ]);
    assert_eq!(opts.paths().len(), 3);
    assert_eq!(opts.paths()[0], "./config");
    assert_eq!(opts.paths()[1], "/etc/myapp");
    assert_eq!(opts.paths()[2], "/usr/local/etc/myapp");
}

#[test]
fn test_options_set_paths_replaces() {
    let mut opts = Options::default();
    opts.add_path("./old");
    assert_eq!(opts.paths().len(), 1);

    opts.set_paths(vec!["./new1".to_string(), "./new2".to_string()]);
    assert_eq!(opts.paths().len(), 2);
    assert_eq!(opts.paths()[0], "./new1");
    assert_eq!(opts.paths()[1], "./new2");
}

#[test]
fn test_options_set_project() {
    let mut opts = Options::default();
    assert_eq!(opts.project(), "");

    opts.set_project("myapp");
    assert_eq!(opts.project(), "myapp");
}

#[test]
fn test_options_set_project_replaces() {
    let mut opts = Options::with_project("oldapp");
    assert_eq!(opts.project(), "oldapp");

    opts.set_project("newapp");
    assert_eq!(opts.project(), "newapp");
}

#[test]
fn test_options_builder_pattern() {
    let mut opts = Options::with_project("myapp");
    opts.add_path("./config")
        .add_path("/etc/myapp")
        .set_project("newapp");

    assert_eq!(opts.project(), "newapp");
    assert_eq!(opts.paths().len(), 2);
}

#[test]
fn test_options_equality() {
    let opts1 = Options::with_project("myapp");
    let opts2 = Options::with_project("myapp");
    let opts3 = Options::with_project("other");

    assert_eq!(opts1, opts2);
    assert_ne!(opts1, opts3);
}

#[test]
fn test_options_clone() {
    let mut opts1 = Options::with_project("myapp");
    opts1.add_path("./config");

    let opts2 = opts1.clone();
    assert_eq!(opts1, opts2);
    assert_eq!(opts2.project(), "myapp");
    assert_eq!(opts2.paths().len(), 1);
}
