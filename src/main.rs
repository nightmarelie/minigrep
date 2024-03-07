use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    println!("{:?}", args);
}
