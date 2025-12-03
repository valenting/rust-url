use url::Url;

#[test]
fn test_set_hostname_removes_normalization() {
    // Test: <non-spec:/.//p> set hostname to <h>
    let mut url = Url::parse("non-spec:/.//p").unwrap();
    println!("Initial URL: {}", url.as_str());
    println!("Initial path: {}", url.path());
    println!("Initial host: {:?}", url.host());

    url.set_host(Some("h")).unwrap();

    println!("\nAfter set_host(Some('h')):");
    println!("  URL: {}", url.as_str());
    println!("  Path: {}", url.path());
    println!("  Host: {:?}", url.host());

    // When setting a hostname, the URL now has authority, so "/." normalization should be removed
    assert_eq!(url.as_str(), "non-spec://h//p");
    assert_eq!(url.path(), "//p");
}

#[test]
fn test_set_hostname_empty_removes_normalization() {
    // Test: <non-spec:/.//p> set hostname to <>
    let mut url = Url::parse("non-spec:/.//p").unwrap();
    println!("Initial URL: {}", url.as_str());
    println!("Initial host: {:?}", url.host());
    println!("Initial has_authority: {}", url.has_authority());

    let result = url.set_host(Some(""));
    println!("\nset_host result: {:?}", result);

    println!("\nAfter set_host(Some('')):");
    println!("  URL: {}", url.as_str());
    println!("  Path: {}", url.path());
    println!("  Host: {:?}", url.host());
    println!("  has_authority: {}", url.has_authority());

    // For non-special schemes, empty hostname is allowed and creates an empty host
    // The WPT test expects "non-spec:////p" which has authority ("//") with empty host and path "//p"
    assert_eq!(url.as_str(), "non-spec:////p");
    assert_eq!(url.path(), "//p");
}
