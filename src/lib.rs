extern crate serde;

use serde::Deserialize;
use std::{env, error, fs, path::Path};
use toml;

pub const CONFIG: &str = "config.toml";

pub fn copy_dotfiles(config: Config) -> Result<(), Box<dyn error::Error>> {
    for file in config.dotfiles {
        let source_path = env::var("HOME").unwrap() + "/" + &file;

        let mut target_path = file.chars(); // create an iterator from str slice
        target_path.next(); // skip the first element, that is actually a dot

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
pub struct Config {
    dotfiles: Vec<String>,
    target: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, Box<dyn error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn target_file_exists() -> Result<(), Box<dyn error::Error>> {
        let path = Path::new("/Users/srj_b/.test_file1.txt");

        File::create(&path)?;

        if path.exists() {
            let config = Config {
                dotfiles: vec![String::from(".test_file1.txt")],
                target: env::var("HOME").unwrap() + "/",
            };

            copy_dotfiles(config)?
        }

        assert!(Path::new("/Users/srj_b/test_file1.txt").exists());

        Ok(())
    }
    // TODO:
    // - Get rid from hardcoded paths
    // - Implement cleanup after the test
}
