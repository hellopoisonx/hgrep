use std::{env, error::Error, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            is_sensitive: env::var("CASE_SENSITIVE").is_ok(),
        })
    }
}
pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(c.file_path)?;
    match c.is_sensitive {
        true => {
            for line in search_sensitive(&c.query, &content) {
                println!("{}", line);
            }
        }
        false => {
            for line in search_insensitive(&c.query, &content) {
                println!("{}", line);
            }
        }
    }
    Ok(())
}
pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "hello";
        let contents = r"\
Rust:
hello
Hello
                        ";
        assert_eq!(vec!["hello"], search_sensitive(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "Hello";
        let contents = r"\
Rust:
hello
Hello
                        ";
        assert_eq!(vec!["hello", "Hello"], search_insensitive(query, contents))
    }
}
