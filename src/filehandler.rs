use std::collections::HashMap;
use encrypt;
use std::fs::OpenOptions;
use std::fs::File;
use std::io;
use std::io::prelude::Write;
use std::io::prelude::Read;
use rpassword::prompt_password_stdout;

pub fn save_account_file(accounts: &HashMap<String, String>) {
    let mut result = String::new();

    for (acc, token) in accounts {
        result.push_str(&format!("{}:{}\n", acc, token)[..]);
    }

    let password = prompt_password_stdout("Password: ").unwrap();
    let (encrypted_content, iv) = encrypt::encrypt_content(&result[..], &password[..]);
    let content = format!("{}:{}\n", encrypted_content, iv);
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(".account.txt")
        .expect("unable to create or open file.");
    file.write_all(content.as_bytes())
        .expect("unable to write to account.txt");
}

pub fn load_account_file() -> HashMap<String, String> {
    let mut file = File::open(".account.txt").expect("could not open account file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read account file");
    if contents.len() < 1 {
        return HashMap::new();
    }
    let split: Vec<&str> = contents.split(":").collect();
    let content = split[0];
    println!("content: {}", content);
    let iv = split[1].trim_right();
    println!("iv: {}", iv);
    let password = prompt_password_stdout("Password: ").unwrap();
    let decrypted_content = encrypt::decrypt_content(content, &password[..], iv);
    HashMap::new()
}
