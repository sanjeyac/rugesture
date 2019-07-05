mod keys;
mod configuration;
mod gesture;

use configuration::read_config;
use gesture::run_debug_input;

fn main(){
    
    keys::key_press("alt+Tab".to_string());
    let config = read_config(&"Settings.toml".to_string());
    run_debug_input();

}
