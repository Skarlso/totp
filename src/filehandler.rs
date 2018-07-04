use encrypt;
use rpassword::prompt_password_stderr;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
// use std::io;
use std::io::prelude::Read;
use std::io::prelude::Write;

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
            .read(true)
            .write(true)
            .create(true)
            .open(".account.txt")
            .expect("unable to create or open file.");
        file.write_all(content.as_bytes())
            .expect("unable to write to account.txt");
    }

    pub fn load_account_file(&mut self) {
        let mut file = File::open(".account.txt").expect("could not open account file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read account file");
        if contents.len() < 1 {
            return;
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
    }

    pub fn get_token(&self, acc: String) -> String {
        self.accounts[&acc].to_owned()
    }
}
