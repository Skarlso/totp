use chrono::{Local};
use data_encoding::BASE32;
use crypto::{hmac::Hmac, sha1::Sha1};
use crypto::mac::{Mac, MacResult};
use byteorder::{ByteOrder, BigEndian};

pub fn generate_otp_token(token: String) -> String {
    let now = Local::now().timestamp();
    let timer = (now / 30) as u32;
    let secret_bytes = BASE32.encode(token.as_bytes());
    let mut buf = [0; 8];
    let mut hm = Hmac::new(Sha1::new(), &secret_bytes.as_bytes());
    BigEndian::write_u32(&mut buf, timer);
    hm.input(&buf[..]);

    let res = hm.output_bytes();
    println!("bytes: {}", res);
    String::from(secret_bytes)
}
