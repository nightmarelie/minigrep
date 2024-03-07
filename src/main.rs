use std::env;
use std::fs;

fn main() {
    // by default it will return the path of the binary
    // ["target/debug/minigrep"]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    // the first argument is the path of the binary. By index 0 we will ignore it.
    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("{:?}", args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With contents: {}", contents);
}
