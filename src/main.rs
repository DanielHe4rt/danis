// Uncomment this block to pass the first stage


use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(handle_client(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


async fn handle_client(mut stream: TcpStream) {
    {
        let mut buffer = [0; 1024];

        println!("stream: {:?}", stream);
        loop {
            let read_count = stream.read(&mut buffer).await.unwrap();
            if read_count == 0 {
                break;
            }
            println!("read_count: {:?}", read_count);

            stream.write(b"+PONG\r\n").await.unwrap();
        }

        println!("received data: {:?}", buffer);
    }
}