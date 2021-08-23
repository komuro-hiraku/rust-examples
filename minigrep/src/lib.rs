use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

// public function
pub fn run(config: Config) -> Result<(), Box<Error>> {
    // file not found
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // for line in search(&config.query, &contents) {
    //     println!("{}", line);
    // }

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
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
    let query = query.to_lowercase();   // 全部小文字変換
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {   // 小文字変換して比較
            results.push(line); // 元のlineは変わってないのでそのまま
        }
    }
    
    results
}

// Config
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // fn parse_config(args: &[String]) -> Config をリファクタ
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";
        
        assert_eq!(vec!["safe, fast, productive."],
                    search(query, contents),
                );
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
            search_case_insensitive(query, contents),
        );
    }

}