mod libs;

use std::env;
use std::fs;
use std::error::Error;
use std::process::exit;

use crate::libs::Config;
use crate::libs::search;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file)?;

    search(&config.query, &contents)
        .iter()
        .for_each(|line: &&str| {
            println!("{line}")
        });

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(||{
        println!("Problem parsing arguments");
        exit(1)
    } );

    if let Err(e) = run(config) {
        println!("Application Error : {}", e);
        exit(7)
    }
}
