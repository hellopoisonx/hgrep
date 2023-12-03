use ansi_term::Color::{White, Red};
use std::{env, error::Error, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_sensitive: bool,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments"),
        };
        let file_path = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments"),
        };
        Ok(Config {
            is_sensitive: env::var("CASE_SENSITIVE").is_ok(),
            query,
            file_path,
        })
    }
}
pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(c.file_path)?;
    match c.is_sensitive {
        true => {
            for line in search_sensitive(&c.query, &content) {
                let index = line.find(&c.query).unwrap_or_default();
                let index_end = index + c.query.len();
                println!(
                    "{}{}{}",
                    White.paint(&line[..index]),
                    Red.paint(&line[index..index_end]),
                    White.paint(&line[index_end..])
                );
            }
        }
        false => {
            for line in search_sensitive(&c.query, &content) {
                let index = line.find(&c.query).unwrap_or_default();
                let index_end = index + c.query.len();
                println!(
                    "{}{}{}",
                    White.paint(&line[..index]),
                    Red.paint(&line[index..index_end]),
                    White.paint(&line[index_end..])
                );
            }
        }
    }
    Ok(())
}
pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(&query)).collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query))
        .collect()
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
