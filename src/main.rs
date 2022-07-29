use std::io::prelude::*;
use std::{str, net};
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

    let path = &path[..];
    let port = urls::get_port(host);
    let resp = request(host, path, port);
    show(resp.unwrap());
}

fn request(host: &str, path: &str, port: u16) 
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

fn show(body: String) {

    let mut in_bracket: bool = false;
    let mut in_body: bool = false;
    let mut current_tag: Vec<u8> = vec![];
    let bytes = body.as_bytes();
    let mut clean_text: Vec<u8> = vec![];

    for (_i, &item) in bytes.iter().enumerate() {
        if item == b'<' {
            in_bracket = true;
        } else if item == b'>' {
            in_bracket = false;
            let current_tag_name = str::from_utf8(&current_tag).unwrap();

            if current_tag_name.eq("body") {
               in_body = true; 
            } else if current_tag_name.eq("/body") {
               in_body = false; 
            }

            current_tag = vec![];
        } else if in_bracket {
            current_tag.push(item);
        } else if !in_bracket && in_body {
            clean_text.push(item);
        }
    }

    print!("{}", str::from_utf8(&clean_text).unwrap());
}
