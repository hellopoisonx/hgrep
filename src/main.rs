use hgrep::run;
use hgrep::Config;
use std::env;
fn main() {
    let mut args = env::args();
    let c = {
        let this = Config::new(&mut args);
        match this {
            Ok(t) => t,
            Err(e) => (|err| {
        eprintln!("Error parsing arguments: {}", err);
        std::process::exit(1);
    })(e),
        }
    };
    // println!("query: {}, file_path: {}", c.query, c.file_path);
    if let Err(e) = run(c) {
        eprintln!("Error reading the file {}", e);
        std::process::exit(1);
    };
}
