extern crate serde;

use serde::Deserialize;
use std::{env, error, fs};
use toml;

fn main() -> Result<(), Box<dyn error::Error>> {
    // read a list of params
    let args: Vec<String> = env::args().skip(1).collect();

    // TODO:
    // - Add error handling
    // - Pull out all to separate functions, structures etc
    // - Move all stuff to lib.rs
    // - Implement tests
    // - Unbind args from hardcoded args[1] position

    println!("{:?}", Config::new(&args)?); // DEBUG

    Ok(())
}

// bring the Deserialize trait
// which transforms string into a struct
#[derive(Deserialize, Debug)]
struct Config {
    dotfiles: Vec<String>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, Box<dyn error::Error>> {
        let mut dotfiles = vec![];

        for arg in args.iter() {
            if arg == "config.toml" {
                // read configuration file into a string
                let config_str = fs::read_to_string(&arg)?;

                // get .toml structure from string
                let config: Config = toml::from_str(&config_str)?;

                dotfiles = config.dotfiles; // shadows previous empty `dotfiles` var
            } else {
                return Err(Box::from("config.toml was not provided"));
            }
        }

        Ok(Config { dotfiles })
    }
}
