// https://doc.rust-jp.rs/book-ja/ch12-01-accepting-command-line-arguments.html

extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}



