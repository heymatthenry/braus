use std::io::prelude::*;
use std::net;
use clap::{Arg, App};

mod urls;

fn main() {
    let matches = App::new("braüs")
                        .arg(Arg::with_name("url"))
                        .help("URL to load")
                        .get_matches();

    let url = matches.value_of("url").unwrap();
    let (_scheme, url) = urls::parse_url(url);

    let (host, path) = urls::get_host_and_path(url);

    let port = urls::get_port(host);
    let resp = request(host, path, port);
    print!("{}", resp.unwrap());
}

fn request(host: &str, path: String, port: u16) 
                -> std::io::Result<String> 
{
    let mut socket = net::TcpStream::connect((host, port))?;
    let body = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.write_all(body.as_bytes())?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

