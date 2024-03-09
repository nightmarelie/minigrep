use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // the first argument is the path of the binary. By index 0 we will ignore it.
        let query = args[1].clone();
        let filename = args[2].clone();

        println!("{:?}", args);

        println!("Searching for {}", query);
        println!("In file {}", filename);

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("The result is: ");

    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if case_sensitive {
            if line.contains(query) {
                results.push(line);
            }
        } else {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("filename"),
        ];
        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive.", "Duct tape."], search(query, contents, false));
    }
}
