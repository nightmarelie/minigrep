use std::env;
use std::fs;

fn main() {
    // by default it will return the path of the binary
    // ["target/debug/minigrep"]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let (query, filename) = parse_config(&args);

    let contents = read_contents_from_file(filename);
}

fn parse_config(args: &[String]) -> (&String, &String) {
    // the first argument is the path of the binary. By index 0 we will ignore it.
    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("{:?}", args);

    println!("Searching for {}", query);
    println!("In file {}", filename);
    (query, filename)
}

fn read_contents_from_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    contents
}
