use std::process::Command;

pub fn key_press(keys: String) {
    Command::new("xdotool")
        .arg("key")
        .arg(keys)
        .spawn()
        .expect("failed to execute process"); 
}

