use std::collections::HashMap;
use encrypt;
use std::fs::OpenOptions;
use std::fs::File;
use std::io;
use std::io::prelude::Write;
use std::io::prelude::Read;

pub fn save_account_file(accounts: HashMap<String, String>) {
    let mut result = String::new();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("could not read line");

    for (acc, token) in accounts {
        result.push_str(&format!("{}:{}\n", acc, token)[..]);
    }

    let (encrypted_content, iv) = encrypt::encrypt_content(&result[..], &password[..]);
    let content = format!("{}:{}\n", encrypted_content, iv);
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(".account.txt")
        .expect("unable to create or open file.");
    file.write_all(content.as_bytes())
        .expect("unable to write to account.txt");
}

pub fn load_account_file() -> HashMap<String, String> {
    let mut file = File::open(".account.txt").expect("could not open account file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read account file");
    let split: Vec<&str> = contents.split(":").collect();
    let content = split[0];
    let iv = split[1];
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("could not read line");
    let decrypted_content = encrypt::decrypt_content(content, &password[..], iv);
    HashMap::new()
}