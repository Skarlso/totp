use encrypt;
use std::io;
use std::fs::File;
use std::io::prelude::*;

pub fn save_account(acc: &String, token: &String) {
    println!("password: ");
    let mut password = String::new();
    match io::stdin().read_line(&mut password) {
        Ok(_) => (),
        Err(error) => println!("error: {}", error),
    }
    let content = format!("{}:{}", acc, token);
    let encrypted_content = encrypt::encrypt_content(&content[..], &password[..]);
    let mut file = File::create("account.txt").unwrap();
    match file.write_all(encrypted_content.as_bytes()) {
        Ok(_) => (),
        Err(err) => println!("failed to write file: {}", err),
    }
}

pub fn load_token(acc: &str) -> Option<&str> {
    Some(acc)
}

pub fn delete_account(acc: &String) {}
