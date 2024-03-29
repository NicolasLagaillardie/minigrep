use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Get the args from cmd line
    let args: Vec<String> = env::args().collect();

    // Get the first and second arg
    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Print the two args
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // Handling the possible Error from run()
    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {}", e);

        process::exit(1);
    }
}
