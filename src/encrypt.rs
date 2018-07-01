use openssl::rand;
use openssl::symm;
use openssl::aes::{AesKey, KeyError, aes_ige};

pub fn encrypt(content: String, password: String) -> Vec<u8> {
    let mut con = String::from("#CHECKME#");
    con.push_str("#CHECKME#");
    // The password will be used to generate a key
    let mut password = password.as_bytes().to_vec();
    let cipher = symm::Cipher::aes_256_ctr();
    // Zero pedding to aes_256 key length
    while password.len() < cipher.key_len() {
        password.push(b'0');
    }
    println!("password: {:?}", password);
    let iv = {
        let mut buf = vec![0; cipher.iv_len().unwrap()];
        rand::rand_bytes(buf.as_mut_slice()).unwrap();
        buf
    };
    println!("iv: {:?}", iv);
    // let key = AesKey::new_encrypt(&password[..]);
    // let mut output = vec![0u8; cipher.key_len()];
    // aes_ige(&, &mut output, &key, &mut iv_as_u8, Mode::Encrypt);
    let encrypted_content =
        symm::encrypt(cipher,
                      &password,
                      Some(iv.as_slice()),
                      content.as_bytes()).unwrap();

    encrypted_content
}

pub fn decrypt(content: String, password: String) -> String {
    String::new()
}
