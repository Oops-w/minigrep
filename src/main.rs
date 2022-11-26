use minigrep;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mini_grep_config = minigrep::MiniGrepConfig::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argusments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(mini_grep_config) {
        println!("mini grep running fail:{:?}", err);
        process::exit(1);
    };
}
