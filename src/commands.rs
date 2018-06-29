extern crate clap;
use std::io::{self, Read};

pub fn add_account() {
    println!("Adding account");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(())
}

pub fn delete_account(account: &str) {
    println!("Deleting account {}", account);
}

pub fn generate_token(account: &str) {
    println!("{:?}", account);
}