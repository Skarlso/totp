use encrypt;
use rpassword::prompt_password_stderr;
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::prelude::Write;
use std::io::{self, ErrorKind};

pub struct FileHandler<'a> {
    accounts: HashMap<String, String>,
    password: String,
    file_name: &'a str,
}

impl<'a> FileHandler<'a> {
    pub fn init() -> Self {
        let password = prompt_password_stderr("Password: ").unwrap();
        Self {
            accounts: HashMap::new(),
            password: password,
            file_name: ".account.txt",
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

        let (encrypted_content, iv) =
            encrypt::encrypt_content(&result[..], &self.password[..]).unwrap();
        let content = format!("{}:{}\n", encrypted_content, iv);
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&self.file_name)
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
            .open(&self.file_name)
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
        let decrypted_content = encrypt::decrypt_content(content, &self.password[..], iv)?;
        let accounts: Vec<&str> = decrypted_content.split("\n").collect();
        for acc in accounts {
            if acc.len() < 1 {
                continue;
            }
            let acc_split: Vec<&str> = acc.split(":").collect();
            if acc_split[0] == encrypt::GOTP_KEY {
                continue;
            }
            &self
                .accounts
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::path::Path;

    #[test]
    fn test_add_account() {
        let mut fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: ".test_account.txt",
        };
        fh.add_account(String::from("new_account"), String::from("newtoken"));
        assert!(fh.accounts.contains_key(&String::from("new_account")));
        assert_eq!(
            String::from("newtoken"),
            fh.get_token(String::from("new_account")).unwrap()
        );
    }

    #[test]
    fn test_get_token_fails_if_account_does_not_exist() {
        let fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: ".test_account.txt",
        };
        let token = fh.get_token(String::from("non_exitant"));
        assert!(token.is_err());
    }

    #[test]
    fn test_save_accountfile() {
        let file_name = ".save_account_test_account.txt";
        let mut fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: file_name,
        };
        fh.add_account(String::from("account"), String::from("token"));
        fh.save_account_file();
        assert!(Path::new(file_name).exists());
        remove_file(file_name).expect("it's all okay");
    }

    #[test]
    fn test_load_accountfile() {
        let file_name = ".load_account_test_account.txt";
        let mut fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: file_name,
        };
        fh.add_account(String::from("account"), String::from("token"));
        fh.save_account_file();
        assert!(Path::new(file_name).exists());
        fh.accounts.remove("account");
        fh.load_account_file()
            .expect("failed to load in account file");
        assert_eq!(
            String::from("token"),
            fh.get_token(String::from("account")).unwrap()
        );
        remove_file(file_name).expect("it's all okay");
    }

    #[test]
    fn test_load_account_also_creates_empty_test_file() {
        let file_name = ".create_test_account_file.txt";
        let mut fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: file_name,
        };
        fh.load_account_file()
            .expect("failed to load in account file");
        assert!(Path::new(file_name).exists());
        remove_file(file_name).expect("it's all okay");
    }

    #[test]
    fn test_delete_account() {
        let file_name = ".delete_account_test_account.txt";
        let mut fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: file_name,
        };
        fh.add_account(String::from("account"), String::from("token"));
        fh.save_account_file();
        assert!(Path::new(file_name).exists());
        let res = fh.delete_account(String::from("account"));
        assert!(res.is_ok());
        remove_file(file_name).expect("it's all okay");
    }

    #[test]
    fn test_delete_account_error_if_account_not_exists() {
        let file_name = ".delete_error_account_test_account.txt";
        let mut fh = FileHandler {
            password: String::from("password"),
            accounts: HashMap::new(),
            file_name: file_name,
        };
        fh.save_account_file();
        assert!(Path::new(file_name).exists());
        let res = fh.delete_account(String::from("account"));
        assert!(res.is_err());
        remove_file(file_name).expect("it's all okay");
    }
}
