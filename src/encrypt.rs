use openssl::symm::{decrypt, encrypt, Cipher};
use data_encoding::BASE64;

pub fn encrypt_content(content: &str, password: &str) -> String {
  let cipher = Cipher::aes_256_cbc();
  let mut password = password.as_bytes().to_vec();
  while password.len() < cipher.key_len() {
    password.push(b'0');
  }
  let data = content.as_bytes();
  let key = password.as_slice();

  let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
  let encrypted_content = encrypt(
      cipher,
      key,
      Some(iv),
      data).unwrap();

  BASE64.encode(encrypted_content.as_slice())
}

pub fn decrypt_content(content: &str, password: &str) -> String {
  let base64_decoded_content = BASE64.decode(content.as_bytes()).unwrap();
  let cipher = Cipher::aes_256_cbc();
  let mut password = password.as_bytes().to_vec();
  while password.len() < cipher.key_len() {
    password.push(b'0');
  }
  let data = base64_decoded_content.as_slice();
  let key = password.as_slice();
  let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
  let decrypted_content = decrypt(
      cipher,
      key,
      Some(iv),
      data).unwrap();
  String::from_utf8(decrypted_content).unwrap()
}
