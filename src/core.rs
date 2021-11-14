// This module provides core functionality such as copy configuration files to
// a target directory, `Config` struct and its constructor

extern crate clap;
extern crate serde;

use clap::ArgMatches;
use serde::Deserialize;
use std::{env, error, fs, path::Path};
use toml;

pub const CONFIG: &str = "config.toml";

pub fn copy_dotfiles(config: Config, args: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    for file in config.dotfiles {
        let source_path = env::var("HOME").unwrap() + "/" + &file;

        let file_base_name = Path::new(&file)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        // Get path's base name as `&OsStr` type and convert it to `String`
        // e.g. /home/user/gnupg/gpg.conf get only gpg.conf name

        let mut target_path = file_base_name.chars(); // create an iterator from str slice

        if config.keep_original_target == false && file_base_name.starts_with('.') {
            target_path.next(); // skip a first element, that is actually a dot
        }

        if Path::new(&source_path).is_dir() {
            eprintln!("warning: {} is a directory", file);
            continue;
        }

        if Path::new(&source_path).exists() {
            fs::copy(
                &source_path,
                // create subslice from original str slice with .as_str method
                String::from(&config.target) + "/" + &target_path.as_str(),
            )?;

            if args.is_present("verbosity") {
                print!("copying {}.. ", &file);
            }
        } else {
            eprintln!("warning: {} file doesn't exists", file);
            continue;
        }

        if args.is_present("verbosity") {
            println!("done");
        }
    }

    if !args.is_present("verbosity") {
        println!("dotfiles successfully copied");
    }

    Ok(())
}

// bring the Deserialize trait which transforms string into a struct
#[derive(Deserialize, Debug)]
pub struct Config {
    // Config struct represents congig.toml file structure
    dotfiles: Vec<String>,
    target: String,
    keep_original_target: bool,
}

impl Config {
    pub fn new(args: &ArgMatches) -> Result<Config, Box<dyn error::Error>> {
        match args.value_of("CONFIG") {
            Some(v) => {
                if Path::new(v).ends_with(CONFIG) {
                    // read configuration file into a string
                    let config_str = fs::read_to_string(v)?;

                    // get .toml structure from string
                    let config: Config = toml::from_str(&config_str)?;

                    let mut keep_original_target = args.is_present("orig_target");

                    if config.keep_original_target == true
                        && args.is_present("orig_target") == false
                    {
                        keep_original_target = config.keep_original_target;
                    }

                    return Ok(Config {
                        dotfiles: config.dotfiles,
                        target: config.target,
                        keep_original_target,
                    });
                } else if Path::new(v).exists() && !Path::new(v).ends_with(CONFIG) {
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
                keep_original_target: false,
            };

            fs::copy(
                &source_path,
                String::from(&config.target) + "/test_file1.txt",
            )?;
        }

        let target_path = Path::new(env::var("HOME").unwrap().as_str()).join("test_file1.txt");
        assert!(target_path.exists());

        fs::remove_file(source_path)?;
        fs::remove_file(target_path)?;

        Ok(())
    }
}
