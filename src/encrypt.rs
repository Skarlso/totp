use data_encoding::BASE64;
use openssl::rand;
use openssl::symm::{decrypt, encrypt, Cipher};

pub fn encrypt_content(content: &str, password: &str) -> (String, String) {
    let cipher = Cipher::aes_256_cbc();
    let mut password = password.as_bytes().to_vec();
    while password.len() < cipher.key_len() {
        password.push(b'0');
    }
    let data = content.as_bytes();
    let key = password.as_slice();

    let iv = {
        let mut buf = vec![0; cipher.iv_len().unwrap()];
        rand::rand_bytes(buf.as_mut_slice()).unwrap();
        buf
    };

    let encrypted_content = encrypt(cipher, key, Some(iv.as_slice()), data).unwrap();
    (
        BASE64.encode(encrypted_content.as_slice()),
        BASE64.encode(iv.as_slice()),
    )
}

pub fn decrypt_content(content: &str, password: &str, iv: &str) -> String {
    let base64_decoded_content = BASE64.decode(content.as_bytes()).unwrap();
    let iv_decoded = BASE64.decode(iv.as_bytes()).unwrap();
    let cipher = Cipher::aes_256_cbc();
    let mut password = password.as_bytes().to_vec();
    while password.len() < cipher.key_len() {
        password.push(b'0');
    }
    let data = base64_decoded_content.as_slice();
    let key = password.as_slice();
    let decrypted_content = decrypt(cipher, key, Some(iv_decoded.as_slice()), data).unwrap();
    String::from_utf8(decrypted_content).unwrap()
}
