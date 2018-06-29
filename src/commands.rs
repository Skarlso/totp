extern crate clap;
use std::io;

pub fn add_account() {
    println!("Adding account");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
}

pub fn delete_account(account: &str) {
    println!("Deleting account {}", account);
}

pub fn generate_token(account: &str) {
    println!("{:?}", account);
}
