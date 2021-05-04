extern crate clap;
extern crate serde;

use bakdf::Config;
use clap::{App, Arg};
use std::{env, error, process};

const CONFIG: &str = "config.toml";

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = App::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name(CONFIG)
                .index(1)
                .help("Set configuration file"),
        )
        .arg(
            Arg::with_name("orig_target")
                .short("k")
                .help("Keep original target name"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .help("Increase verbosity"),
        )
        .get_matches();

    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem with configuration: {}", e);
        process::exit(1);
    });

    if let Err(e) = bakdf::copy_dotfiles(config, &args) {
        eprintln!("error: {} contains invalid elements in its fields", CONFIG);
        eprintln!("{}", e);
        process::exit(1);
    };

    Ok(())
}
