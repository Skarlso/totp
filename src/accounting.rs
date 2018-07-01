use encrypt;

pub fn save_account(acc: String, token: String, password: String) {
    let content = format!("{}:{}", acc, token);
    let encrypted_content = encrypt::encrypt(content, password);
    println!("{:?}", encrypted_content);
}

pub fn load_token(acc: String) -> Option<String> {

    Some(String::from(acc))
}

pub fn delete_account(acc: String) {

}