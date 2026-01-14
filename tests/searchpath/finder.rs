use confyg::searchpath::{find_file, Finder, FinderError, Options};

#[test]
fn test_find_file_defaults() {
    let file = find_file("Cargo.toml", &Options::default()).unwrap();
    assert_eq!(file, "Cargo.toml");
    let err = find_file("cicd.yml", &Options::default());
    assert!(err.is_err());
    assert_eq!(
        err.unwrap_err(),
        FinderError::NotFound("cicd.yml".to_string())
    );
}

#[test]
fn test_find_file_with_paths() {
    let mut opts = Options::default();
    opts.add_path(".github/workflows");
    let file = find_file("ci.yml", &opts).unwrap();
    assert_eq!(file, ".github/workflows/ci.yml");
}

#[test]
fn test_find_file_new() {
    let file = find_file("Cargo.toml", Finder::new().options()).unwrap();
    assert_eq!(file, "Cargo.toml");
    let err = find_file("cicd.yml", Finder::new().options());
    assert!(err.is_err());
    assert_eq!(
        err.unwrap_err(),
        FinderError::NotFound("cicd.yml".to_string())
    );
}

#[test]
fn test_find_file_add_path() {
    let file = Finder::new()
        .add_path(".github/workflows")
        .find("ci.yml")
        .unwrap();
    assert_eq!(file, ".github/workflows/ci.yml");
}

#[test]
fn test_find_file_add_paths() {
    let file = Finder::new()
        .add_path("src/env")
        .add_path(".github/workflows")
        .find("ci.yml")
        .unwrap();
    assert_eq!(file, ".github/workflows/ci.yml");
}

#[test]
fn test_finder_options_mut() {
    let mut finder = Finder::new();
    finder.options_mut().add_path("./test");

    assert_eq!(finder.options().paths().len(), 1);
    assert_eq!(finder.options().paths()[0], "./test");
}

#[test]
fn test_finder_default() {
    let finder = Finder::default();
    assert_eq!(finder.options().paths().len(), 0);
}

#[test]
fn test_finder_clone() {
    let mut finder1 = Finder::new();
    finder1.add_path("./config");

    let finder2 = finder1.clone();
    assert_eq!(finder2.options().paths().len(), 1);
}

#[test]
fn test_finder_debug() {
    let finder = Finder::new();
    let debug_str = format!("{:?}", finder);
    assert!(debug_str.contains("Finder"));
}
