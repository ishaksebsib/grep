use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parshing arguments : {}", err);
        process::exit(1)
    });

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

impl<'a> Config<'a> {
    fn new(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments ;(");
        }

        let query = &args[1];
        let file_name = &args[2];

        Ok(Config { query, file_name })
    }
}
