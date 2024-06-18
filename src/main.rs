use minigrep::Input;

use std::env;
use std::process;
fn main() {

    let args:Vec<String> = env::args().collect();
    eprintln!("{:?}", args);

    let input = Input::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file: {}", input.query, input.file);
    println!("Result:");
    println!("-----------------------------------");

    if let Err(e) = minigrep::run(input) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

