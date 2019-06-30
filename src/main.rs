use std::process::Command;

fn key_press(keys: String) {
    Command::new("xdotool")
        .arg("key")
        .arg(keys)
        .spawn()
        .expect("failed to execute process"); 
}


fn main() {
    key_press("alt+Tab".to_string());
}

