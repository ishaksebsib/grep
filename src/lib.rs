use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments ;(");
        }

        let query = &args[1];
        let file_name = &args[2];

        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!(
        "\n---Searching {} in File Content ---\n\n{}",
        config.query, contents
    );

    Ok(())
}
