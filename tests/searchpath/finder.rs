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
    let opts = Options {
        paths: vec![".github/workflows".to_string()],
    };
    let file = find_file("cicd.yml", &opts).unwrap();
    assert_eq!(file, ".github/workflows/cicd.yml");
}

#[test]
fn test_find_file_new() {
    let file = find_file("Cargo.toml", &Finder::new().opts).unwrap();
    assert_eq!(file, "Cargo.toml");
    let err = find_file("cicd.yml", &Finder::new().opts);
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
        .find("cicd.yml")
        .unwrap();
    assert_eq!(file, ".github/workflows/cicd.yml");
}

#[test]
fn test_find_file_add_paths() {
    let file = Finder::new()
        .add_path("src/env")
        .add_path(".github/workflows")
        .find("cicd.yml")
        .unwrap();
    assert_eq!(file, ".github/workflows/cicd.yml");
}
