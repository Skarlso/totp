// use openssl::rand;
// use openssl::symm;
// use openssl::aes::{AesKey, KeyError, aes_ige};

// pub fn encrypt(content: &String, password: &String) -> (Vec<u8>, Vec<u8>) {
//     let mut con = String::from("#CHECKME#");
//     con.push_str("#CHECKME#");
//     // The password will be used to generate a key
//     let mut password = password.as_bytes().to_vec();
//     let cipher = symm::Cipher::aes_256_ctr();
//     // Zero pedding to aes_256 key length
//     while password.len() < cipher.key_len() {
//         password.push(b'0');
//     }
//     println!("password: {:?}", password);
//     let iv = {
//         let mut buf = vec![0; cipher.iv_len().unwrap()];
//         rand::rand_bytes(buf.as_mut_slice()).unwrap();
//         buf
//     };
//     println!("iv: {:?}", iv);
//     // let key = AesKey::new_encrypt(&password[..]);
//     // let mut output = vec![0u8; cipher.key_len()];
//     // aes_ige(&, &mut output, &key, &mut iv_as_u8, Mode::Encrypt);
//     let encrypted_content =
//         symm::encrypt(cipher,
//                       &password,
//                       Some(iv.as_slice()),
//                       content.as_bytes()).unwrap();

//     (encrypted_content, iv)
// }

// pub fn decrypt(content: &String, password: &String, iv: &Vec<u8>) -> String {
//     let cipher = symm::Cipher::aes_256_ctr();
//     let mut password = password.as_bytes().to_vec();
//     while password.len() < cipher.key_len() {
//         password.push(b'0');
//     }
//     // let iv = {
//     //     let mut buf = vec![0; cipher.iv_len().unwrap()];
//     //     rand::rand_bytes(buf.as_mut_slice()).unwrap();
//     //     buf
//     // };

//     let decrypted_content = symm::decrypt(cipher, &password, Some(iv.as_slice()), content.as_bytes());
//     match decrypted_content {
//         Ok(c) => {
//             let result = String::from_utf8_lossy(c.as_slice());
//             result.into_owned()
//         }
//         Err(err) => panic!("unable to decrypt content {}", err),
//     }
// }
use openssl::aes::{AesKey, KeyError, aes_ige};
use openssl::symm::Mode;
use hex::{FromHex, ToHex};
use openssl::symm;

pub fn encrypt(content: &str, password: &str) -> Vec<u8> {
    let hex_cipher = symm::Cipher::aes_256_ctr();
    let mut password = password.as_bytes().to_vec();
    while password.len() < hex_cipher.key_len() {
        password.push(b'0');
    }
    let randomness = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F";
    if let (Ok(key_as_u8), Ok(cipher_as_u8), Ok(mut iv_as_u8)) =
        (Vec::from_hex(password), Vec::from_hex(hex_cipher.as_ptr()), Vec::from_hex(randomness)) {
            let key = AesKey::new_encrypt(&key_as_u8)?;
            let mut output = vec![0u8; cipher_as_u8.len()];
            aes_ige(&cipher_as_u8, &mut output, &key, &mut iv_as_u8, Mode::Encrypt);
        }
}