use std::io;
use generator;
use accounting;

pub fn add_account() {
    println!("Adding account");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    accounting::save_account(&String::from("test"), &String::from("token"), &String::from("password"));
}

pub fn delete_account(account: &str) {
    println!("Deleting account {}", account);
}

pub fn generate_token(account: &str) {
    // Get the token for the given account here
    let token = accounting::load_token(&account);
    let token = match token {
        Some(t) => t,
        None => panic!("No token found for account: {}", account),
    };
    let otp = generator::generate_otp_token(&token);
    // let otp = generator::generate_otp_token(String::from("MFZWIZQ="));
    println!("{}", otp);
}
