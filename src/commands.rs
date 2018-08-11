use accounting;
use generator;
use std::error::Error;
use std::io::{self, ErrorKind};

pub fn add_account() -> Result<(), Box<Error>> {
    println!("account: ");
    let mut account = String::new();
    io::stdin()
        .read_line(&mut account)
        .expect("unable to read line");
    if account.trim_right().is_empty() {
        return Err(io::Error::new(ErrorKind::InvalidInput, "Please provide an account with at least one character.").into());
    }
    println!("token: ");
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .expect("unable to read line");
    if token.trim_right().is_empty() {
        return Err(io::Error::new(ErrorKind::InvalidInput, "Please provide a token with at least one character.").into());
    }
    accounting::save_account(&account, &token)?;
    Ok(())
}

pub fn delete_account(account: &str) -> Result<(), Box<Error>> {
    println!("Deleting account {}", account);
    accounting::delete_account(&account.to_owned())?;
    Ok(())
}

pub fn generate_token(account: &str) -> Result<(), Box<Error>> {
    // Get the token for the given account here
    let token = accounting::load_token(&account)?;
    let otp = generator::generate_otp_token(&token);
    match otp {
        Ok(t) => {
            println!("{}", t);
            Ok(())
        }
        Err(e) => return Err(e),
    }
}
