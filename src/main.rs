
mod configuration;

use configuration::read_config;

fn main(){
    let config = read_config(&"Settings.toml".to_string());
}