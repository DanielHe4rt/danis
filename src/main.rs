// Uncomment this block to pass the first stage

use std::fmt::Debug;
use anyhow::{anyhow, Result};
use tokio::net::{TcpListener, TcpStream};

use crate::resp::handler::RespHandler;
use crate::resp::parser::RespType;

mod resp;


#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

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


async fn handle_client(stream: TcpStream) -> Result<()> {
    println!("stream: {:?}", stream);

    let mut handler = RespHandler::new(stream);

    loop {
        let response = handler.read_value().await?;

        if let Some(response) = response {
            let (command, args) = extract_value(response)?;

            let value = match command.as_str() {
                "PING" => RespType::SimpleString(String::from("PONG")),
                "ECHO" => args.first().unwrap().clone(),
                _ => { todo!("aqui viado") }
            };

            handler.write_value(value).await?
        }
    }
}

fn extract_value(value: RespType) -> Result<(String, Vec<RespType>)> {
    match value {
        RespType::Array(content) => {
            let command = match content.first().clone() {
                Some(RespType::BulkString(s)) => Ok(s.clone()),
                _ => { Err(anyhow!("eae")) }
            }.unwrap();
            let args = content.into_iter().skip(1).collect();

            Ok((command, args))
        }
        _ => { todo!("deu merda") }
    }
}