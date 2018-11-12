use data_encoding::BASE64;
use openssl::rand;
use openssl::symm::{decrypt, encrypt, Cipher};
use std::error::Error;
use std::io::{self, ErrorKind};

pub const GOTP_KEY: &str = "GOTP";
pub const GOTP_VALUE: &str = "DECRYPT_SUCCESS";

pub fn encrypt_content(content: &str, password: &str) -> Result<(String, String), Box<Error>> {
    let cipher = Cipher::aes_256_cbc();
    let mut password = password.as_bytes().to_vec();
    while password.len() < cipher.key_len() {
        password.push(b'0');
    }

    let content = content.to_owned() + format!("\n{}:{}", GOTP_KEY, GOTP_VALUE).as_str();
    let data = content.as_bytes();
    let key = password.as_slice();

    let iv = {
        let mut buf = vec![0; cipher.iv_len().unwrap()];
        rand::rand_bytes(buf.as_mut_slice()).unwrap();
        buf
    };

    let encrypted_content = encrypt(cipher, key, Some(iv.as_slice()), data)?;
    Ok((
        BASE64.encode(encrypted_content.as_slice()),
        BASE64.encode(iv.as_slice()),
    ))
}

pub fn decrypt_content(content: &str, password: &str, iv: &str) -> Result<String, Box<Error>> {
    let base64_decoded_content = BASE64.decode(content.as_bytes())?;
    let iv_decoded = BASE64.decode(iv.as_bytes())?;
    let cipher = Cipher::aes_256_cbc();
    let mut password = password.as_bytes().to_vec();
    while password.len() < cipher.key_len() {
        password.push(b'0');
    }
    let data = base64_decoded_content.as_slice();
    let key = password.as_slice();
    let decrypted_content = decrypt(cipher, key, Some(iv_decoded.as_slice()), data);
    let decrypted_content = match decrypted_content {
        Ok(s) => s,
        Err(_) => return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid password.").into()),
    };
    let decrypted_content = String::from_utf8(decrypted_content).unwrap();
    if !decrypted_content.contains(format!("\n{}:{}", GOTP_KEY, GOTP_VALUE).as_str()) {
        return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid password.").into());
    }
    Ok(decrypted_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_content() {
        let content = "asdf";
        let password = "password";
        let (encrypted, iv) = encrypt_content(content, password).unwrap();
        assert_ne!(encrypted, content);
        assert!(iv.len() > 0);
        let decrypted = decrypt_content(encrypted.as_str(), password, iv.as_str());
        match decrypted {
            Ok(v) => assert_eq!(v, "asdf\nGOTP:DECRYPT_SUCCESS"),
            Err(_) => panic!("was not excepting an error here"),
        }
    }

    #[test]
    fn test_encrypt_content_bad_password() {
        let content = "asdf";
        let password = "password";
        let (encrypted, iv) = encrypt_content(content, password).unwrap();
        assert_ne!(encrypted, content);
        assert!(iv.len() > 0);
        let decrypted = decrypt_content(encrypted.as_str(), "a", iv.as_str());
        match decrypted {
            Ok(_) => panic!("this should have failed."),
            Err(_) => (),
        }
    }

    #[test]
    fn test_encrypt_content_bad_base32_data() {
        let content = "asdf";
        let password = "password";
        let (encrypted, iv) = encrypt_content(content, password).unwrap();
        assert_ne!(encrypted, content);
        assert!(iv.len() > 0);
        let decrypted = decrypt_content(encrypted.as_str(), password, "not_a_token");
        match decrypted {
            Ok(_) => panic!("this should have failed."),
            Err(_) => (),
        }
    }
}
