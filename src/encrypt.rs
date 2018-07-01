use openssl::rand;
use openssl::symm;

pub fn encrypt(content: String, password: String) -> String {
    let mut con = String::from("#CHECKME#");
    con.push_str("#CHECKME#");
    // The password will be used to generate a key
    let mut password = password.as_bytes().to_vec();
    let cipher = symm::Cipher::aes_256_ctr();
    // Zero pedding to aes_256 key length
    while password.len() < cipher.key_len() {
        password.push(b'0');
    }

    let iv = {
        let mut buf = vec![0; cipher.iv_len().unwrap()];
        rand::rand_bytes(buf.as_mut_slice());
        buf
    };
    let encrypted_content =
        symm::encrypt(cipher,
                      &password,
                      Some(iv.as_slice()),
                      content.as_bytes()).unwrap();

    match String::from_utf8(encrypted_content) {
        Ok(res) => res,
        Err(error) => panic!("can't convert encrypted content: {}", error),
    }
}

pub fn decrypt(content: String, password: String) -> String {
    String::new()
}
