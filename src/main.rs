use clap::{Arg, App};
mod urls;

fn main() {
    let matches = App::new("braüs")
                        .arg(Arg::with_name("url"))
                        .help("URL to load")
                        .get_matches();

    let url = matches.value_of("url").unwrap();
    let (scheme, url) = urls::parse_url(url);

    let (host, path) = urls::get_host_and_path(url);
    println!("scheme: {}", scheme);
    println!("host: {}", host);
    println!("path: {}", path);
}

