use std::fs;
use std::error::Error;

// Create a struct for more flexibility
pub struct Config{
    pub query: String,
    pub filename: String,
}

// Implement methods for the struct
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        // Return an error if not enough args
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Divide the args and assign them into the struct
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query,filename})
    }
}

// Catch logic of the project
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Convert the content of the file into String
    let contents = fs::read_to_string(config.filename)?;

    // Print each line which contains the query
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // Return Ok() if everything when good
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

// Return either an empty Vec or the query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
