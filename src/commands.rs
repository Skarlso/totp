use accounting;
use generator;
use std::io;
use std::process;

pub fn add_account() {
    println!("account: ");
    let mut account = String::new();
    io::stdin()
        .read_line(&mut account)
        .expect("unable to read line");
    if account.trim_right().is_empty() {
        println!("Please provide an account with at least one character.");
        process::exit(1);
    }
    println!("token: ");
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .expect("unable to read line");
    if token.trim_right().is_empty() {
        println!("Please provide a token with at least one character.");
        process::exit(1);
    }
    accounting::save_account(&account, &token);
}

pub fn delete_account(account: &str) {
    println!("Deleting account {}", account);
    accounting::delete_account(&account.to_owned());
}

pub fn generate_token(account: &str) {
    // Get the token for the given account here
    let token = accounting::load_token(&account);
    let otp = generator::generate_otp_token(&token);
    println!("{}", otp);
}
