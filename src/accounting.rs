pub fn save_account(acc: &String, token: &String) {
    println!("password: ");
    // Check if account already exists
    // Add it if not
    // Pass it to file handler to save it
    let content = format!("{}:{}", acc, token);


}

pub fn load_token(acc: &str) -> Option<&str> {
    Some(acc)
}

pub fn delete_account(acc: &String) {}
