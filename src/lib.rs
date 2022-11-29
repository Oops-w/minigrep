use std::error::Error;
use std::{env, fs};

pub struct MiniGrepConfig {
    pub query_str: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl MiniGrepConfig {
    pub fn new(mut args: env::Args) -> Result<MiniGrepConfig, String> {
        // ignore program name
        args.next();

        let query_str = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("query string is null")),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("file path is null")),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query_str: query_str,
            file_path: file_path,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(mini_grep_config: MiniGrepConfig) -> Result<(), Box<dyn Error>> {
    let conetnts = fs::read_to_string(mini_grep_config.file_path)?;

    let results = if mini_grep_config.case_sensitive {
        search(&mini_grep_config.query_str, &conetnts)
    } else {
        search_case_insensitive(&mini_grep_config.query_str, &conetnts)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
