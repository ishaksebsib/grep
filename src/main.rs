use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let mut f = File::open(config.file_name).expect("File not Found ;(");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading this file ;(");

    println!(
        "\n---Searching {} in File Content ---\n\n{}",
        config.query, contents
    );
}

struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = &args[1];
    let file_name = &args[2];

    Config { query, file_name }
}
