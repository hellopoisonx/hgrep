use hgrep::run;
use hgrep::Config;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let c = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        std::process::exit(1);
    });
    // println!("query: {}, file_path: {}", c.query, c.file_path);
    if let Err(e) = run(c) {
        eprintln!("Error reading the file {}", e);
        std::process::exit(1);
    };
}
