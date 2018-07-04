use filehandler;

pub fn save_account(acc: &String, token: &String) {
    // Check if account already exists
    // Add it if not
    // Pass it to file handler to save it
    let mut accounts = filehandler::load_account_file();
    accounts.insert(acc.to_owned(), token.to_owned());
    filehandler::save_account_file(&accounts);
}

pub fn load_token(acc: &str) -> Option<&str> {
    Some(acc)
}

pub fn delete_account(acc: &String) {}
