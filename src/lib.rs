use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments ;(");
        }

        let query = &args[1];
        let file_name = &args[2];
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let result = if config.case_sensitive {
        case_insensetive_search(&config.query, &contents)
    } else {
        case_insensetive_search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn case_sensetive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub fn case_insensetive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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

        assert_eq!(
            vec!["safe, fast, productive."],
            case_sensetive_search(query, contents)
        )
    }

    #[test]
    fn test_case_insensitivity() {
        let query = "BODY";
        let contents = "body";

        assert_eq!(vec!["body"], case_insensetive_search(query, contents))
    }

    #[test]
    fn test_case_sensitivity() {
        let query = "BODY";
        let contents = "body";

        let expected_result: Vec<&str> = Vec::new();
        assert_eq!(expected_result, case_sensetive_search(query, contents))
    }
}
