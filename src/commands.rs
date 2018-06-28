extern crate clap;

pub fn add_account() {

}

pub fn delete_account() {

}

pub fn generate_token(args: &clap::ArgMatches<'_>) {
    println!("{:?}", args.value_of("account").unwrap());
}