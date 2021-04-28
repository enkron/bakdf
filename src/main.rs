//extern crate clap;
extern crate serde;

//use clap::{App, Arg};
use std::{env, error, process};
use tmp::Config;

const CONFIG: &str = "config.toml";

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(env::args())?;

    if env::args().count() == 1 {
        eprintln!("error: The program do nothing without args");
        process::exit(1);
    }

    if let Err(_) = tmp::copy_dotfiles(config) {
        eprintln!("error: Cannot read target field in the {}", CONFIG);
        process::exit(1);
    };

    //    let matches = App::new("tmp")
    //        .author(clap::crate_authors!("\n"))
    //        .arg(Arg::with_name("config.toml").last(true))
    //        .get_matches();
    //
    //    println!("{:?}", matches);

    Ok(())
}
