extern crate serde;

use serde::Deserialize;
use std::{env, error, fs, path::Path, process};
use toml;

const CONFIG: &str = "config.toml";

fn main() -> Result<(), Box<dyn error::Error>> {
    // TODO:
    // - Add error handling
    // - Pull out all to separate functions, structures etc
    // - Move all stuff to lib.rs
    // - Implement tests
    // - Unbind args from hardcoded args[1] position [V]
    // - Add a target as command line option and create a new field
    //   in the Config struct

    let config = Config::new(env::args())?;

    if let Err(_) = copy_dotfiles(config) {
        eprintln!("error: Cannot read target field in the {}", CONFIG);
        process::exit(1);
    };

    Ok(())
}

fn copy_dotfiles(config: Config) -> Result<(), Box<dyn error::Error>> {
    for file in config.dotfiles {
        let source_path = env::var("HOME").unwrap() + "/" + &file;

        let mut target_path = file.chars(); // create an iterator from str slice
        target_path.next(); // skip first element

        if Path::new(&source_path).exists() {
            fs::copy(
                &source_path,
                // create subslice from original str slice with .as_str method
                String::from(&config.target) + "/" + &target_path.as_str(),
            )?;

            print!("copying {}.. ", &file);
        } else {
            eprintln!("warning: {} file doesn't exists.", file);
            continue;
        }

        println!("done");
    }

    Ok(())
}

// bring the Deserialize trait
// which transforms string into a struct
#[derive(Deserialize, Debug)]
struct Config {
    dotfiles: Vec<String>,
    target: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, Box<dyn error::Error>> {
        args.next();
        let mut dotfiles = vec![];
        let mut target = String::new();

        for arg in args {
            if arg == CONFIG {
                // read configuration file into a string
                let config_str = fs::read_to_string(&arg)?;

                // get .toml structure from string
                let config: Config = toml::from_str(&config_str)?;

                dotfiles = config.dotfiles; // shadows previous empty `dotfiles` var
                target = config.target;
            } else {
                return Err(Box::from("config.toml was not provided"));
            }
        }

        Ok(Config { dotfiles, target })
    }
}
