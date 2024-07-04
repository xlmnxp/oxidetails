use std::{
    io::{Read, Write},
    net::TcpListener,
};
use crate::config_manager;

pub fn start() {
    println!("Web Server Started on [::]:{}", config_manager::get_value("listen_on"));

    let tcp_listener =
        TcpListener::bind(format!(":::{}", config_manager::get_value("listen_on")))
        .expect("Cannot listen on specified port");

    for stream in tcp_listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]);

        if request.starts_with("GET /.well-known/oxidetalis/public HTTP/1.1") {
            let response = format!(
                "HTTP/1.1 200 OK\r\n\r\n{}",
                config_manager::get_value("public_key")
                    .as_str()
                    .unwrap()
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            continue;
        }

        if request.starts_with("GET /.well-known/oxidetalis/server HTTP/1.1") {
            let response = format!(
                "HTTP/1.1 200 OK\r\n\r\n{}",
                format!("{{\"otmp_server\": {}}}", config_manager::get_value("server_name"))
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            continue;
        }

        let response = "HTTP/1.1 200 OK\r\n\r\nHello, This is Oxidetails Public Server!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
