use std::env;
use std::process;

// our library import
use minigrep::Config;

fn main() {
    // env::args returns an iterator
    // instead of using collect to get them into a vector, we can just use env::args
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
