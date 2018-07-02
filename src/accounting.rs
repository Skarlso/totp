use encrypt;

pub fn save_account(acc: &String, token: &String, password: &String) {
    let content = format!("{}:{}", acc, token);
    let encrypted_content = encrypt::encrypt(&content, &password);
    println!("{:?}", encrypted_content);
    let decrypted = encrypt::decrypt(&content, &password, &iv);
    println!("decrypted: {}", decrypted);
}

pub fn load_token(acc: &str) -> Option<&str> {

    Some(acc)
}

pub fn delete_account(acc: &String) {

}