use serde::Deserialize;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub three_finger_swipe: Swipe,
    pub four_finger_swipe: Swipe,
}

#[derive(Debug, Deserialize)]
pub struct Swipe {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String
}

pub fn read_config(filename: &String) -> Result<Config,Box<Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let decoded: Config = toml::from_str(&contents)?;
    return Ok(decoded);
}
