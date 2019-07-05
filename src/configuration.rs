use serde::Deserialize;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;


#[derive(Debug, Deserialize)]
pub struct Config {
    three_finger_swipe: Swipe,
    four_finger_swipe: Swipe,
}

#[derive(Debug, Deserialize)]
pub struct Swipe {
    up: String,
    down: String,
    left: String,
    right: String
}

pub fn read_config(filename: &String) -> Result<Config,Box<Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let decoded: Config = toml::from_str(&contents)?;
    return Ok(decoded);
}
