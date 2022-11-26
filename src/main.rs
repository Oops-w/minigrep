use std::{env, fs, process};

struct MiniGrepConfig {
    query_str: String,
    file_name: String,
}

impl MiniGrepConfig {
    fn new(args: &[String]) -> Result<MiniGrepConfig, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"))
        }
        Ok(Self {
            query_str: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mini_grep_config = MiniGrepConfig::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing argusments: {}", err);
            process::exit(1);
        });
    
    println!("query str: {}", mini_grep_config.query_str);

    run(mini_grep_config);
}

fn run(mini_grep_config: MiniGrepConfig) {
    let conetnts = fs::read_to_string(mini_grep_config.file_name)
        .expect("Something went wrong reading the file");

    println!("read context: {}", conetnts);
}