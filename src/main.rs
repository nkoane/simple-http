use std::io::{BufRead, Read, Write};

fn main() {
    let host = "127.0.0.1:7707";
    match std::net::TcpListener::bind(host) {
        Ok(listener) => {
            println!("Listening on port http://{}", host);
            for mut stream in listener.incoming().flatten() {
                let mut request = std::io::BufReader::new(&mut stream);
                let mut headers: Vec<String> = Vec::new();
                loop {
                    let mut header = String::new();

                    match request.read_line(&mut header) {
                        Ok(0) => break,
                        Ok(_) => {
                            headers.push(header.clone());
                            if header.trim().is_empty() {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            break;
                        }
                    }
                    // print!("{header}");
                }

                println!("Headers: {:#?}", headers);

                // check if the first entry is a GET request
                #[derive(Debug)]
                enum Method {
                    GET,
                    POST,
                    PUT,
                    DELETE,
                    PATCH,
                    OPTIONS,
                    HEAD,
                    CONNECT,
                    TRACE,
                }

                const VERSION: &str = "HTTP/1.1";

                let mut parts = headers[0].split_whitespace();

                // check if method and version are valid
                let method = match parts.next() {
                    Some("GET") => Method::GET,
                    Some("POST") => Method::POST,
                    Some("PUT") => Method::PUT,
                    Some("DELETE") => Method::DELETE,
                    Some("PATCH") => Method::PATCH,
                    Some("OPTIONS") => Method::OPTIONS,
                    Some("HEAD") => Method::HEAD,
                    Some("CONNECT") => Method::CONNECT,
                    Some("TRACE") => Method::TRACE,
                    _ => {
                        stream
                            .write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n")
                            .unwrap();
                        continue;
                    }
                };

                let path = parts.next().unwrap();
                let version = parts.next().unwrap();

                if version != VERSION {
                    stream
                        .write_all(b"HTTP/1.1 505 HTTP Version Not Supported\r\n\r\n")
                        .unwrap();
                    continue;
                }

                println!("Method: {:?}, Path: {}, Version: {}", method, path, version);

                match method {
                    Method::GET => {
                        if path == "/" {
                            let mut path = std::path::PathBuf::new(); //from("public_html/index.html");
                            path.push("public_html");
                            path.push("index.html");
                            println!("{:?}", path);

                            if path.exists() {
                                let mut file = std::fs::File::open(path).unwrap(); // handle errors
                                let mut contents = Vec::new();
                                file.read_to_end(&mut contents).unwrap(); // handle errors;
                                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                                stream.write_all(contents.as_slice()).unwrap();
                            } else {
                                stream
                                    .write_all(b"HTTP/1.1 404 Not Found\r\n\r\nFile not found")
                                    .unwrap();
                            }
                        } else if path == "/madume" {
                            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nMadume!").unwrap();
                        } else {
                            stream
                                .write_all(b"HTTP/1.1 404 Not Found\r\n\r\nUnknown request path")
                                .unwrap();
                        }
                    }
                    _ => {
                        stream
                            .write_all(
                                b"HTTP/1.1 501 Not Implemented\r\n\r\nMethod not implemented",
                            )
                            .unwrap();
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
