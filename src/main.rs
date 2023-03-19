mod libs;

use std::env;
use std::fs;
use std::error::Error;
use std::process::exit;

use libs::search_case_insensitive;

use crate::libs::Config;
use crate::libs::search;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file)?;

    if config.case {
        search_case_insensitive(&config.query, &contents)
        .iter()
        .for_each(|line| {
            println!("{line}")
        });
    } else {
        search(&config.query, &contents)
            .iter()
            .for_each(|line: &&str| {
                println!("{line}")
            });
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(||{
        eprintln!("Problem parsing arguments");
        exit(1)
    } );

    if let Err(e) = run(config) {
        eprintln!("Application Error : {}", e);
        exit(7)
    }
}
