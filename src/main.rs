mod urls;

fn main() {
    let url = "http://example.com";
    let (scheme, url) = urls::parse_url(url);

    let (host, path) = urls::get_host_and_path(url);
    println!("{}", host);
    println!("{}", path);
}

