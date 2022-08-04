use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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
    let path = if url_parts.is_empty() {
        String::from("/")
    } else { 
        url_parts.join("/") 
    };

    (host, path)
}

pub fn read_file(path: &str) -> std::io::Result<String> {
    let path = String::from("/") + path;
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// If there's a port specified along with the hostname
/// (i.e. localhost:3000), pull them apart
pub fn get_port(_host: &str) -> u16 {
    80
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
