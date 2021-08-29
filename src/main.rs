extern crate clap;
extern crate serde;

use bakdf::Config;
use clap::{App, Arg, ArgMatches};
use std::{env, error, process};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(&arg_parse()).unwrap_or_else(|e| {
        eprintln!("Problem with configuration: {}", e);
        process::exit(1);
    });

    if let Err(e) = bakdf::copy_dotfiles(config, &arg_parse()) {
        eprintln!(
            "error: {} contains invalid elements in its fields",
            bakdf::CONFIG
        );
        eprintln!("{}", e);
        process::exit(1);
    };

    Ok(())
}

fn arg_parse() -> ArgMatches<'static> {
    App::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name(bakdf::CONFIG)
                .index(1)
                .default_value("/home/bsa/etc/bakdf/config.toml")
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
                .long("verbose")
                .help("Increase verbosity"),
        )
        .get_matches()
}
