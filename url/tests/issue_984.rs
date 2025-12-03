use url::Url;

#[test]
fn test_issue_984_set_path() {
    // Test Case 1: Using set_path()
    let mut url = Url::parse("web+demo:/").unwrap();
    println!("Initial URL: {}", url.as_str());

    url.set_path("//not-a-host");
    println!("After set_path: {}", url.as_str());
    println!("Path: {}", url.path());

    // The path should be "//not-a-host"
    assert_eq!(url.path(), "//not-a-host", "Path should preserve leading slashes");

    // To prevent "//not-a-host" from being interpreted as an authority,
    // the URL should serialize with "/." prepended
    assert_eq!(url.as_str(), "web+demo:/.//not-a-host", "URL should prepend '/.' to prevent authority interpretation");
}

#[test]
fn test_issue_984_path_segments_mut() {
    // Test Case 2: Using path_segments_mut()
    let mut url = Url::parse("web+demo:/").unwrap();

    url.path_segments_mut()
        .expect("should have path segments")
        .extend(&["", "not-a-host"]);

    println!("After path_segments_mut: {}", url.as_str());
    println!("Path: {}", url.path());

    // The path should be "//not-a-host" (empty segment + "not-a-host" = //not-a-host)
    assert_eq!(url.path(), "//not-a-host", "Path should be //not-a-host");

    // The URL should serialize with "/." to prevent authority interpretation
    assert_eq!(url.as_str(), "web+demo:/.//not-a-host", "URL should prepend '/.' to prevent authority interpretation");
}

#[test]
fn test_issue_984_with_host() {
    // When a URL has a host, double slashes should be preserved without normalization
    let mut url = Url::parse("http://example.com/").unwrap();

    url.path_segments_mut()
        .expect("should have path segments")
        .extend(&["", "path"]);

    // Should have double slashes in the path
    assert_eq!(url.as_str(), "http://example.com//path");
}
