use filehandler::FileHandler;

pub fn save_account(acc: &String, token: &String) {
    let mut fh = FileHandler::init();
    fh.load_account_file();
    // check if account exists before adding.
    fh.add_account(acc.trim_right().to_owned(), token.trim_right().to_owned());
    fh.save_account_file();
}

pub fn load_token(acc: &str) -> String {
    let mut fh = FileHandler::init();
    fh.load_account_file();
    fh.get_token(acc.trim_right().to_owned())
}

pub fn delete_account(acc: &String) {
    let mut fh = FileHandler::init();
    fh.load_account_file();
    // check if account exists before adding.
    fh.delete_account(acc.trim_right().to_owned());
    fh.save_account_file();
}
