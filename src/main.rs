mod keys;
mod configuration;

use configuration::read_config;

fn main(){
    keys::key_press("alt+Tab".to_string());
    let config = read_config(&"Settings.toml".to_string());
}
