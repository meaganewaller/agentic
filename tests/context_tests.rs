#[test]
fn hostname_is_normalized() {
    let name = agentic::context::detect_hostname().unwrap();
    assert!(!name.contains('.'));
    assert_eq!(name, name.to_lowercase());
}
