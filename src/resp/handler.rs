use anyhow::Result;
use bytes::BytesMut;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::resp::parser::{RespParser, RespType};

pub struct RespHandler {
    pub stream: TcpStream,
    buffer: BytesMut,
    parser: RespParser,
}


impl RespHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(512),
            parser: RespParser::new(),
        }
    }

    pub async fn read_value(&mut self) -> Result<Option<RespType>>
    {
        let read_bytes_count = self.stream.read_buf(&mut self.buffer).await.unwrap();

        if read_bytes_count == 0 {
            return Ok(None);
        }

        let (value, _) = self.parser.transform(self.buffer.split())?;

        Ok(Some(value))
    }

    pub async fn write_value(&mut self, payload: RespType) -> Result<()> {
        self.stream.write(payload.serialize().as_bytes()).await.unwrap();

        Ok(())
    }
}