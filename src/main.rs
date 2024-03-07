use std::env;

fn main() {
    // by default it will return the path of the binary
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    println!("{:?}", args);
}
