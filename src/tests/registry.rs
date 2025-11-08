use crate::registry::{RegistryError, id::{is_core_id, normalize_id}};

#[test]
fn normalize_basic() {
    assert_eq!(normalize_id("Hello.World").unwrap(), "hello.world");
}

#[test]
fn reject_empty() {
    let err = normalize_id("     ").unwrap_err();
    assert!(matches!(err, RegistryError::InvalidId(_)));
}

#[test]
fn reject_bad_char() {
    let err = normalize_id("abc$def").unwrap_err();
    assert!(matches!(err, RegistryError::InvalidId(_)));
}

#[test]
fn allow_single_colon_middle() {
    assert_eq!(normalize_id("ns:thing").unwrap(), "ns:thing");
}

#[test]
fn reject_colon_at_ends() {
    assert!(normalize_id(":abc").is_err());
    assert!(normalize_id("abc:").is_err());
}

#[test]
fn reject_second_colon() {
    assert!(normalize_id("a:b:c").is_err());
}

#[test]
fn boundary_lengths() {
    assert!(normalize_id("a").is_ok());
    let long = "a".repeat(200);
    assert!(normalize_id(&long).is_ok());
    let too_long = "a".repeat(201);
    assert!(normalize_id(&too_long).is_err());
}

#[test]
fn core_detection() {
    assert!(is_core_id("core:foo"));
    assert!(!is_core_id("corex:foo"));
}
