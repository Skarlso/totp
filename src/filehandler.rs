use encrypt;
use rpassword::prompt_password_stderr;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::prelude::Write;
use std::error::Error;
use std::io::{self, ErrorKind};

pub struct FileHandler {
    accounts: HashMap<String, String>,
    password: String,
}

impl FileHandler {
    pub fn init() -> FileHandler {
        let password = prompt_password_stderr("Password: ").unwrap();
        FileHandler {
            accounts: HashMap::new(),
            password: password,
        }
    }

    pub fn add_account(&mut self, account: String, token: String) {
        &self.accounts.insert(account, token);
    }

    pub fn save_account_file(&self) {
        let mut result = String::new();

        for (acc, token) in &self.accounts {
            result.push_str(&format!("{}:{}\n", acc, token));
        }

        let (encrypted_content, iv) = encrypt::encrypt_content(&result[..], &self.password[..]);
        let content = format!("{}:{}\n", encrypted_content, iv);
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(".account.txt")
            .expect("unable to create or open account file.");
        file.write_all(content.as_bytes())
            .expect("unable to write to account.txt");
    }

    pub fn load_account_file(&mut self) -> Result<(), Box<Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(".account.txt")
            .expect("unable to create or open account file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read account file");
        if contents.len() < 1 {
            return Ok(());
        }
        let split: Vec<&str> = contents.split(":").collect();
        let content = split[0];
        let iv = split[1].trim_right();
        let decrypted_content = encrypt::decrypt_content(content, &self.password[..], iv);
        let accounts: Vec<&str> = decrypted_content.split("\n").collect();
        for acc in accounts {
            if acc.len() < 1 {
                continue;
            }
            let acc_split: Vec<&str> = acc.split(":").collect();
            &self.accounts
                .insert(acc_split[0].to_owned(), acc_split[1].to_owned());
        }
        Ok(())
    }

    pub fn get_token(&self, acc: String) -> Result<String, Box<Error>> {
        if !self.accounts.contains_key(&acc) {
            let s = format!("account {} not found", acc);
            return Err(io::Error::new(ErrorKind::InvalidInput, s).into());
        }
        Ok(self.accounts[&acc].to_owned())
    }

    pub fn delete_account(&mut self, acc: String) -> Result<(), Box<Error>> {
        if !self.accounts.contains_key(&acc) {
            let s = format!("account {} not found", acc);
            return Err(io::Error::new(ErrorKind::InvalidInput, s).into());
        }
        self.accounts.remove(&acc);
        Ok(())
    }
}
