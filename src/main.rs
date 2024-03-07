use std::env;

fn main() {
    // by default it will return the path of the binary
    // ["target/debug/minigrep"]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    // the first argument is the path of the binary. By index 0 we will ignore it.
    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("{:?}", args);
}
