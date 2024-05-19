use anyhow::{anyhow, Result};
use bytes::BytesMut;

#[derive(Debug, Clone)]
pub enum RespType {
    BulkString(String),
    SimpleString(String),
    Array(Vec<RespType>),
}

impl RespType {
    pub fn serialize(self) -> String {
        match self {
            RespType::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            RespType::SimpleString(s) => format!("+{}\r\n", s),
            _ => panic!("Azedou o parsing!")
        }
    }
}


#[derive(Debug)]
pub struct RespParser {}


impl RespParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Loops through the payload until find the `\r\n`
    /// # Examples
    ///
    /// ```
    /// use bytes::{BytesMut, BufMut};
    ///
    /// //
    /// let mut buffer = b'*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n';
    /// let parser = RespParser::new().transform(buffer);
    ///
    pub fn transform(&self, payload: BytesMut) -> Result<(RespType, usize)> {
        let command = payload[0] as char;

        match command {
            '+' => self.parse_string(payload),
            '$' => self.parse_bulk_strings(payload),
            '*' => self.parse_array(payload),
            _ => {
                println!("deu ruim aqui fml");
                todo!("deu ruim aqui fml")
            }
        }
    }

    fn parse_array(&self, payload: BytesMut) -> Result<(RespType, usize)> {
        let response = self.read_until_crlf(&payload[1..]);

        let (array_length, mut bytes_consumed) = if let Some((array_length, bytes_consumed)) = response {
            let array_length = self.parse_int(array_length)?;

            (array_length, bytes_consumed + 1)
        } else {
            return Err(anyhow!("Invalid array format"));
        };

        let mut resp_values = vec![];

        for _ in 0..array_length {
            let (parsed_resp, len) = self.transform(BytesMut::from(&payload[bytes_consumed..]))?;

            resp_values.push(parsed_resp);
            bytes_consumed += len;
        }

        Ok((RespType::Array(resp_values), bytes_consumed))
    }


    fn parse_bulk_strings(&self, payload: BytesMut) -> Result<(RespType, usize)> {
        let response = self.read_until_crlf(&payload[1..]);
        let (bulk_str_len, bytes_consumed) = if let Some((array_length, bytes_consumed)) = response {
            let array_length = self.parse_int(array_length)?;

            (array_length, bytes_consumed + 1)
        } else {
            return Err(anyhow!("Invalid array format"));
        };

        //  b'*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n';
        let end_of_bulk_str = bytes_consumed + bulk_str_len as usize;
        let total_parsed = end_of_bulk_str + 2;
        let parsed_value = String::from_utf8(payload[bytes_consumed..end_of_bulk_str].to_vec())?;

        Ok((RespType::BulkString(parsed_value), total_parsed))
    }
    fn parse_int(&self, payload: &[u8]) -> Result<i64> {
        Ok(String::from_utf8(payload.to_vec()).unwrap().parse::<i64>()?)
    }
    fn read_until_crlf<'a>(&'a self, payload: &'a [u8]) -> Option<(&[u8], usize)> {
        for i in 1..payload.len() {
            if payload[i - 1] == b'\r' && payload[i] == b'\n' {
                return Some((&payload[0..(i - 1)], i + 1));
            }
        }

        None
    }
    fn parse_string(&self, payload: BytesMut) -> Result<(RespType, usize)> {
        if let Some((string_bytes, bytes_consumed)) = self.read_until_crlf(&payload[1..]) {
            let parsed_string = String::from_utf8(string_bytes.to_vec()).unwrap();

            return Ok((RespType::SimpleString(parsed_string), bytes_consumed));
        }

        Err(anyhow!("azedou a string"))
    }
}
