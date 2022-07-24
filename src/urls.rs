pub fn parse_url(url: &str) -> (&str, &str) {
    let schemes = ["http", "https", "file", "data"];
    let url_parts: Vec<&str> = url.split("://").collect();
    let scheme = url_parts[0];
    let url = url_parts[1];

    assert!(schemes.contains(&scheme));

    (scheme, url)
}

pub fn get_host_and_path(url: &str) -> (&str, String) {
    let mut url_parts: Vec<&str> = url.split("/").collect();
    let host = url_parts.remove(0);
    let path = url_parts.join("/");

    (host, path)
}

/// If there's a port specified along with the hostname
/// (i.e. localhost:3000), pull them apart
fn get_port(host: &str) -> (&str, &str) {
    todo!();
}

#[test]
fn test_parse_url() {
    let (scheme, url) = parse_url("http://example.com");
    assert_eq!("http", scheme);
    assert_eq!("example.com", url);
}

#[test]
fn test_get_host_and_path() {
    let (host, path) = get_host_and_path("example.com/foo/bar/baz");
    assert_eq!("example.com", host);
    assert_eq!("foo/bar/baz", path);
}
