use std::io;
use generator;

pub fn add_account() {
    println!("Adding account");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
}

pub fn delete_account(account: &str) {
    println!("Deleting account {}", account);
}

pub fn generate_token(account: &str) {
    // Get the token for the given account here
    let token = generator::generate_otp_token(String::from("MFZWIZQ="));
    println!("{}", token);
}
