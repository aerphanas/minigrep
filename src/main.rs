use std::env;
use std::fs;
use std::error::Error;
use std::process::exit;

use minigrep::Config;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    println!("With text:\n{contents}");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(||{
        println!("Problem parsing arguments");
        exit(1)
    } );

    let content = fs::read_to_string(config.file)
        .unwrap_or_else( |x| {
            println!("{x}");
            exit(2)
        });

    if let Err(e) = run(config) {
        println!("Application Error : {}", e);
        exit(7)
    }
}
