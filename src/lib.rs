extern crate clap;
extern crate serde;

use clap::ArgMatches;
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
    pub fn new(args: ArgMatches) -> Result<Config, Box<dyn error::Error>> {
        match args.value_of(CONFIG) {
            Some(v) => {
                if v == CONFIG {
                    // read configuration file into a string
                    let config_str = fs::read_to_string(v)?;

                    // get .toml structure from string
                    let config: Config = toml::from_str(&config_str)?;

                    //dotfiles = config.dotfiles; // shadows previous empty `dotfiles` var
                    //target = config.target;
                    return Ok(Config {
                        dotfiles: config.dotfiles,
                        target: config.target,
                    });
                } else if Path::new(v).exists() && v != CONFIG {
                    return Err(Box::from("incorrect config file"));
                } else {
                    return Err(Box::from(format!("no such file - {}", v)));
                }
            }

            None => return Err(Box::from("config file was not provided")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn target_file_exists() -> Result<(), Box<dyn error::Error>> {
        let source_path = Path::new(env::var("HOME").unwrap().as_str()).join(".test_file1.txt");
        File::create(&source_path)?;

        if source_path.exists() {
            let config = Config {
                dotfiles: vec![String::from(".test_file1.txt")],
                target: env::var("HOME").unwrap() + "/",
            };

            copy_dotfiles(config)?
        }

        let target_path = Path::new(env::var("HOME").unwrap().as_str()).join("test_file1.txt");
        assert!(target_path.exists());

        fs::remove_file(source_path)?;
        fs::remove_file(target_path)?;

        Ok(())
    }
}