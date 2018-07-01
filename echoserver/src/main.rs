use std::io::prelude::*;

fn main() {
    println!("Starting server...");
    let server = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server is up {:?}", server);
    loop {
        match server.accept() {
            Ok((mut stream, origin)) => {
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).unwrap();
                let str = std::str::from_utf8(&buffer[0..(n - 1)]).unwrap();
                println!("{}, {}\n{}\n===================\n", n, origin, str);
                if str == "exit" {
                    break;
                }
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
    println!("Tearing down server");
}
