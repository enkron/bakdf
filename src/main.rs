extern crate clap;
extern crate serde;

use clap::{App, Arg};
use std::{env, error, process};
use bakdf::Config;

const CONFIG: &str = "config.toml";

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = App::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .arg(Arg::with_name(CONFIG).index(1))
        .get_matches();

    let config = Config::new(args).unwrap_or_else(|e| {
        eprintln!("Problem with configuration: {}", e);
        process::exit(1);
    });

    if let Err(e) = bakdf::copy_dotfiles(config) {
        eprintln!("error: {} contains invalid elements in its fields", CONFIG);
        eprintln!("{}", e);
        process::exit(1);
    };

    Ok(())
}
