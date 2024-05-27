// Uncomment this block to pass the first stage


use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use tokio::net::{TcpListener, TcpStream};

use crate::database::Danis;
use crate::resp::handler::RespHandler;
use crate::resp::parser::RespType;
use crate::resp::parser::RespType::{NullBulkString, SimpleString};

mod resp;
mod database;


#[tokio::main]
async fn main() -> Result<()> {
    let database = Arc::new(Mutex::new(Danis::new()));

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _)) => {
                let db = database.clone();
                tokio::spawn(async move {
                    handle_client(stream, db).await.unwrap();
                }).await?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


async fn handle_client(
    stream: TcpStream,
    database: Arc<Mutex<Danis>>,
) -> Result<()> {
    let mut handler = RespHandler::new(stream);

    loop {
        let response = handler.read_value().await?;

        if let Some(response) = response {
            let (command, args) = extract_value(response)?;

            let value = match command.as_str() {
                "PING" => SimpleString(String::from("PONG")),
                "ECHO" => args.first().unwrap().clone(),
                "SET" => {
                    let key = args.first().unwrap().clone().value();
                    let value = args.last().unwrap().clone();

                    {
                        let mut database = database.lock().unwrap();
                        database.set(key.clone(), value).unwrap();
                    }
                    SimpleString(String::from("OK"))
                }
                "GET" => {
                    let key = args.first().unwrap().clone().value();
                    {
                        let mut database = database.lock().unwrap();
                        database.get(key.clone()).unwrap_or_else(|_| NullBulkString)
                    }
                }
                _ => { todo!("aqui viado") }
            };

            println!("{:?}", value);

            handler.write_value(value).await?
        }

        break;
    }

    Ok(())
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