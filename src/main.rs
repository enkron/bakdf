extern crate serde;

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

    Ok(())
}
