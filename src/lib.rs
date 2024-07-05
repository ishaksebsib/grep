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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
