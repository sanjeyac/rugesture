mod keys;
mod configuration;
mod gesture;

use configuration::*;
use gesture::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseFloatError;


fn press_key_on_gesture( fingers: u8, direction: &GestureDirection, config: &Config ) {

    println!("fingers: {}, gesture {:?}",&fingers,&direction);

    if fingers == 3 {
        match direction {
            GestureDirection::UP => keys::key_press(&config.three_finger_swipe.up),
            GestureDirection::DOWN => keys::key_press(&config.three_finger_swipe.down),
            GestureDirection::LEFT => keys::key_press(&config.three_finger_swipe.left),
            GestureDirection::RIGHT => keys::key_press(&config.three_finger_swipe.right),
            GestureDirection::NONE => ()
        }
    }
    
    if fingers == 4 {
        match direction {
            GestureDirection::UP => keys::key_press(&config.four_finger_swipe.up),
            GestureDirection::DOWN => keys::key_press(&config.four_finger_swipe.down),
            GestureDirection::LEFT => keys::key_press(&config.four_finger_swipe.left),
            GestureDirection::RIGHT => keys::key_press(&config.four_finger_swipe.right),
            GestureDirection::NONE => ()
        }
    }
}

fn execute_command(line: &String, last_update: &mut String, config: &configuration::Config){    
    match compute(line, last_update) {
        Some(gesture) => press_key_on_gesture(gesture.0, &gesture.1, config),
        None => ()
    }
}


fn main() -> Result<(),Box<Error>>{

    // temp variable
    let mut last_update = "".to_string();
    let LIBINPUT_DEBUG_COMMAND = "libinput-debug-events";

    // read the configurations from the settings file
    // each gesture has a key that will be pressed described in the configuration file
    let config = read_config(&"Settings.toml".to_string()).unwrap();
    
    // run lib input command to catch gesture events
    let stdout = Command::new(LIBINPUT_DEBUG_COMMAND)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    // reader of lines of gesture events
    let reader = BufReader::new(stdout);

    // for each gesture event execute a command from the config file
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| execute_command( &line, &mut last_update, &config ));

    Ok(())

}
