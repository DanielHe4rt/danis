use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];

                println!("accepted new connection");
                println!("stream: {:?}", stream);
                loop {
                    let read_count = stream.read(&mut buffer).unwrap();
                    if read_count == 0 {
                        break;
                    }
                    println!("read_count: {:?}", read_count);

                    stream.write(b"+PONG\r\n").unwrap();
                }


                //println!("received data: {:?}", buffer);

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
