use accounting;
use generator;
use std::io;

pub fn add_account() {
    println!("account: ");
    let mut account = String::new();
    io::stdin()
        .read_line(&mut account)
        .expect("unable to read line");
    println!("token: ");
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .expect("unable to read line");
    accounting::save_account(&account, &token);
}

pub fn delete_account(account: &str) {
    println!("Deleting account {}", account);
    accounting::delete_account(&account.to_owned());
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
