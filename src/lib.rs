use std::error::Error;
use std::{fs, env};

pub struct MiniGrepConfig {
    pub query_str: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl MiniGrepConfig {
    pub fn new(args: &[String]) -> Result<MiniGrepConfig, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query_str: args[1].clone(),
            file_name: args[2].clone(),
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(mini_grep_config: MiniGrepConfig) -> Result<(), Box<dyn Error>> {
    let conetnts = fs::read_to_string(mini_grep_config.file_name)?;
    
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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            results.push(line);
        }
    }

    results
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
