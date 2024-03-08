use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // by default, it will return the path of the binary
    // ["target/debug/minigrep"]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // the first argument is the path of the binary. By index 0 we will ignore it.
        let query = args[1].clone();
        let filename = args[2].clone();

        println!("{:?}", args);

        println!("Searching for {}", query);
        println!("In file {}", filename);

        Ok(Config { query, filename })
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
