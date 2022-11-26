use std::error::Error;
use std::fs;

pub struct MiniGrepConfig {
    pub query_str: String,
    pub file_name: String,
}

impl MiniGrepConfig {
    pub fn new(args: &[String]) -> Result<MiniGrepConfig, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }
        Ok(Self {
            query_str: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

pub fn run(mini_grep_config: MiniGrepConfig) -> Result<(), Box<dyn Error>> {
    let conetnts = fs::read_to_string(mini_grep_config.file_name)?;

    for line in search(&mini_grep_config.query_str, &conetnts) {
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
}
