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

// TODO:
// - Add error handling
// - Pull out all to separate functions, structures etc
// - Move all stuff to lib.rs
// - Implement tests
// - Unbind args from hardcoded args[1] position [V]
// - Add a target as command line option and create a new field
//   in the Config struct
