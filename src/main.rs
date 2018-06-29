#[macro_use]
extern crate clap;
use clap::App;

mod generator;
mod encrypt;
mod accounting;
mod commands;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("add", Some(_)) => commands::add_account(),
        ("generate", Some(token)) => {
            match token.value_of("account") {
                Some(acc) => commands::generate_token(acc),
                None => println!("Please define an --account to generate a token for"),
            }
        },
        ("delete", Some(token)) => {
            match token.value_of("account") {
                Some(acc) => commands::delete_account(acc),
                None => println!("Please define an --account to delete"),
            }
        },
        _ => {},
    }
}
