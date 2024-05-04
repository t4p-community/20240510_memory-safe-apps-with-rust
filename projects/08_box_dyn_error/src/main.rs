mod config;

use crate::config::Config;
use std::fs::File;
use std::io;
use std::io::Read;

// pub fn read_config_file(file_path: &str) -> io::Result<String> {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

fn main() {
    // the function read_config_file only outputs one kind of error
    // if let Ok(contents) = read_config_file("config.yml") {
    //     let config: Config = serde_yaml::from_str(&contents).expect("Unable to parse config file.");
    //     println!("Config:\n{}", config);
    // } else {
    //     eprintln!("Error reading config file");
    // }

    // match Config::from_file("config.yml") {
    //     Ok(config) => println!("Config:\n{}", config),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
}
