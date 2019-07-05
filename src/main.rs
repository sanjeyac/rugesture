mod keys;
mod configuration;
mod gesture;

use configuration::*;
use gesture::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseFloatError;

fn execute_command(line: &String, last_update: &mut String, config: &configuration::Config){    
    
    match compute(line, last_update) {
        Some(gesture) => {

            if gesture.0 == 3 {
                match gesture.1 {
                    GestureDirection::UP => keys::key_press(&config.three_finger_swipe.up),
                    GestureDirection::DOWN => keys::key_press(&config.three_finger_swipe.down),
                    GestureDirection::LEFT => keys::key_press(&config.three_finger_swipe.left),
                    GestureDirection::RIGHT => keys::key_press(&config.three_finger_swipe.right),
                    GestureDirection::NONE => ()
                }
            }
            
            if gesture.0 == 4 {
                match gesture.1 {
                    GestureDirection::UP => keys::key_press(&config.four_finger_swipe.up),
                    GestureDirection::DOWN => keys::key_press(&config.four_finger_swipe.down),
                    GestureDirection::LEFT => keys::key_press(&config.four_finger_swipe.left),
                    GestureDirection::RIGHT => keys::key_press(&config.four_finger_swipe.right),
                    GestureDirection::NONE => ()
                }
            }
        },
        None => ()
    }

}


fn main() -> Result<(),Box<Error>>{

    // keys::key_press("alt+Tab".to_string());

    let config = read_config(&"Settings.toml".to_string()).unwrap();


    let mut last_update = "".to_string();

    let LIBINPUT_DEBUG_COMMAND = "libinput-debug-events";

    let stdout = Command::new(LIBINPUT_DEBUG_COMMAND)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| execute_command( &line, &mut last_update, &config ));

    Ok(())

}
