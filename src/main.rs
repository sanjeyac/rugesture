
use serde::Deserialize;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;


#[derive(Debug, Deserialize)]
struct Config {
    three_finger_swipe: Swipe,
    four_finger_swipe: Swipe,
}

#[derive(Debug, Deserialize)]
struct Swipe {
    up: String,
    down: String,
    left: String,
    right: String
}

fn read_config(filename: &String) -> Result<Config,Box<Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let decoded: Config = toml::from_str(&contents)?;
    return Ok(decoded);
}

fn main() {
    let toml_str = r#"
        [three_finger_swipe]
        up = "3-swipe-up"
        down = "3-swipe-down"
        left = "3-swipe-left"
        right = "3-swipe-right"

        [four_finger_swipe]
        up = "4-swipe-up"
        down = "4-swipe-down"
        left = "4-swipe-left"
        right = "4-swipe-right"
    "#;


    let decoded: Config = toml::from_str(toml_str).unwrap();

    let file_path = "Settings.toml".to_string();
    let decoded_from_file: Config = read_config(&file_path).unwrap();

    println!("{:#?}", decoded);
    println!("this is the up action for 4: {}", decoded_from_file.four_finger_swipe.up)
}