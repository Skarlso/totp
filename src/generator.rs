use chrono::{Local};
use data_encoding::BASE32;
use crypto::{hmac::Hmac, sha1::Sha1};
use crypto::mac::{Mac};
use byteorder::{ByteOrder, BigEndian};

pub fn generate_otp_token(token: String) -> String {
    let now = Local::now().timestamp();
    let timer = (now / 30) as u64;
    let secret_bytes = BASE32.decode(token.as_bytes());
    let bytes = match secret_bytes {
        Ok(bytes) => bytes,
        Err(error) => panic!("error occurred while decoding token: {}", error),
    };
    let mut buf = [0; 8];
    let mut hm = Hmac::new(Sha1::new(), &bytes[..]);
    BigEndian::write_u64(&mut buf, timer);
    hm.input(&buf[..]);
    let res = hm.result();
    let result = res.code();
    let offset = match &result.last() {
        Some(l) => *l & 0xf,
        None => panic!("was not able to get last byte of hmac result"),
    };
    // let result = result.to_vec();
    let offset = offset as usize;
    let value = ((((result[offset]) as i32 & 0x7f) << 24) |
                (((result[offset+1]) as i32 & 0xff) << 16) |
                (((result[offset+2]) as i32 & 0xff) << 8) |
                (((result[offset+3]) as i32 & 0xff))) as i64;

    let length = 6;
    let pow10: i64 = 10;
    let modulo = value % pow10.pow(length);
    format!("{:0length$}", modulo, length = 6)
}
