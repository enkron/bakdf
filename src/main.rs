extern crate serde;

use std::{env, error, process};
use tmp::Config;

const CONFIG: &str = "config.toml";

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(env::args())?;

    if let Err(_) = tmp::copy_dotfiles(config) {
        eprintln!("error: Cannot read target field in the {}", CONFIG);
        process::exit(1);
    };

    Ok(())
}
