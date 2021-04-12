extern crate serde;

use serde::Deserialize;
use std::{env, error, fs};
use toml;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    // read a list of params
    let args: Vec<String> = env::args().collect();

    // TODO:
    // - Add error handling
    // - Pull out all to separate functions, structures etc
    // - Move all stuff to lib.rs

    // read configuration file into a string
    let config_str = fs::read_to_string(&args[1])?;
    // get .toml structure from string
    let config: Config = toml::from_str(&config_str)?;

    // debug
    println!("{:?}", config.dotfiles);

    Ok(()) // return the `unit` struct
}

// bring the Deserialize trait which transforms string into the struct
#[derive(Deserialize, Debug)]
struct Config {
    dotfiles: Vec<String>,
}
