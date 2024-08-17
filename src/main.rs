use std::process;

use grrs::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("looking for {} in: {}", config.query, config.filename);

    if let Err(e) = grrs::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
