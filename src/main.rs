#[macro_use]
extern crate clap;
extern crate byteorder;
extern crate chrono;
extern crate crypto;
extern crate data_encoding;
extern crate hex;
extern crate openssl;
extern crate rpassword;

use clap::App;

mod accounting;
mod commands;
mod encrypt;
mod generator;
mod filehandler;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("add", Some(_)) => commands::add_account(),
        ("generate", Some(token)) => match token.value_of("account") {
            Some(acc) => commands::generate_token(acc),
            None => println!("Please define an --account to generate a token for"),
        },
        ("delete", Some(token)) => match token.value_of("account") {
            Some(acc) => commands::delete_account(acc),
            None => println!("Please define an --account to delete"),
        },
        _ => {}
    }
}
