#[macro_use]
extern crate clap;

mod generator;
mod encrypt;
mod accounting;
mod commands;

fn main() {
    use clap::App;
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("add", Some(_)) => commands::add_account(),
        ("generate", Some(token)) => commands::generate_token(token),
        _ => {},
    }
}
