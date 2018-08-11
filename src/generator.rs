use byteorder::{BigEndian, ByteOrder};
use chrono::Local;
use crypto::mac::Mac;
use crypto::{hmac::Hmac, sha1::Sha1};
use data_encoding::BASE32;
use std::error::Error;
use std::io::{self, ErrorKind};

pub fn generate_otp_token(token: &str) -> Result<String, Box<Error>> {
    let now = Local::now().timestamp();
    let timer = (now / 30) as u64;
    let secret_bytes = BASE32.decode(token.as_bytes());
    let bytes = match secret_bytes {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(io::Error::new(ErrorKind::InvalidInput, "token is not a valid base32 data type").into());
        }
    };
    let mut buf = [0; 8];
    let mut hm = Hmac::new(Sha1::new(), &bytes[..]);
    BigEndian::write_u64(&mut buf, timer);
    hm.input(&buf[..]);
    let res = hm.result();
    let result = res.code();
    let offset = match &result.last() {
        Some(l) => *l & 0xf,
        None => {
            return Err(io::Error::new(ErrorKind::InvalidInput, "was not able to get last byte of hmac result").into());
        }
    };
    // let result = result.to_vec();
    let offset = offset as usize;
    let value = ((((result[offset]) as i32 & 0x7f) << 24)
        | (((result[offset + 1]) as i32 & 0xff) << 16)
        | (((result[offset + 2]) as i32 & 0xff) << 8)
        | ((result[offset + 3]) as i32 & 0xff)) as i64;

    let length = 6;
    let pow10: i64 = 10;
    let modulo = value % pow10.pow(length);
    Ok(format!("{:0length$}", modulo, length = 6))
}
