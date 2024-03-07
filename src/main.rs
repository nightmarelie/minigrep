use std::env;
use std::fs;
use std::process;

fn main() {
    // by default, it will return the path of the binary
    // ["target/debug/minigrep"]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    match Config::new(&args) {
        Ok(config) => {
            let contents = read_contents_from_file(&config.filename);
        }
        Err(e) => {
            println!("Problem parsing arguments: {}", e);
        }
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


fn read_contents_from_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    contents
}
