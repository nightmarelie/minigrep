use std::env;
use std::process;
use minigrep::Config;


fn main() {
    // by default, it will return the path of the binary
    // ["target/debug/minigrep"]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
