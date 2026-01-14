use confyg::searchpath::FinderError;
use std::error::Error;

#[test]
fn test_finder_error_not_found() {
    let err = FinderError::NotFound("config.toml".to_string());
    let err_string = format!("{}", err);
    assert!(err_string.contains("config.toml"));
    assert!(err_string.contains("couldn't find file"));
}

#[test]
fn test_finder_error_path_conversion() {
    let err = FinderError::PathConversion("invalid/path".to_string());
    let err_string = format!("{}", err);
    assert!(err_string.contains("invalid/path"));
    assert!(err_string.contains("UTF-8"));
}

#[test]
fn test_finder_error_equality() {
    let err1 = FinderError::NotFound("file1.toml".to_string());
    let err2 = FinderError::NotFound("file1.toml".to_string());
    let err3 = FinderError::NotFound("file2.toml".to_string());

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

#[test]
fn test_finder_error_debug() {
    let err = FinderError::NotFound("test.toml".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("NotFound"));
    assert!(debug_str.contains("test.toml"));
}

#[test]
fn test_finder_error_from_str() {
    let err: FinderError = "missing.toml".into();
    assert_eq!(err, FinderError::NotFound("missing.toml".to_string()));

    let err_string = format!("{}", err);
    assert!(err_string.contains("missing.toml"));
}

#[test]
fn test_finder_error_is_error_trait() {
    let err = FinderError::NotFound("test.toml".to_string());
    // Test that it implements Error trait
    let _err_ref: &dyn Error = &err;

    // Test source() returns None (no underlying error)
    assert!(err.source().is_none());
}

#[test]
fn test_finder_error_path_conversion_is_error() {
    let err = FinderError::PathConversion("bad/path".to_string());
    let _err_ref: &dyn Error = &err;
    assert!(err.source().is_none());
}
