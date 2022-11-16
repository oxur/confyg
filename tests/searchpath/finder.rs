use confyg::searchpath::{find_file, Finder, Options};

#[test]
fn test_find_file_defaults() {
    let file = find_file("Cargo.toml", &Options::default());
    assert_eq!(file, "Cargo.toml");
    let file = find_file("cicd.yml", &Options::default());
    assert_eq!(file, "");
}

#[test]
fn test_find_file_with_paths() {
    let opts = Options{
        paths: vec![".github/workflows".to_string()],

        .. Default::default()
    };
    let file = find_file("cicd.yml", &opts);
    assert_eq!(file, ".github/workflows/cicd.yml");
}

#[test]
fn test_find_file_new() {
    let file = find_file("Cargo.toml", &Finder::new().opts);
    assert_eq!(file, "Cargo.toml");
    let file = find_file("cicd.yml", &Finder::new().opts);
    assert_eq!(file, "");
}

#[test]
fn test_find_file_add_path() {
    let file = Finder::new()
        .add_path(".github/workflows")
        .find("cicd.yml");
    assert_eq!(file, ".github/workflows/cicd.yml");
}

#[test]
fn test_find_file_add_paths() {
    let file = Finder::new()
        .add_path("src/env")
        .add_path(".github/workflows")
        .find("cicd.yml");
    assert_eq!(file, ".github/workflows/cicd.yml");
}
