
use dirs;
use toml;

use std::path::{
  Path, PathBuf
};
use std::fs;
use std::env;

mod config;

fn main() {
    let home_dir = dirs::home_dir().unwrap_or(PathBuf::from("."));
    // The first file in this list which exists is picked
    // as the config file
    let possible_config_files = [
      "sys-stat.toml",
      &format!("{}/.sys-stat.toml", home_dir.display()),
      &format!("{}\\.sys-stat.toml", home_dir.display()),
    ];
    let config_file = get_first_path(&possible_config_files);
    if let Err(_) = config_file {
      println!("Error: none of the following config files exist:");
      println!("{:#?}", possible_config_files);
      return;
    }

    let config_file = config_file.expect("Already checked Err case");
    println!("Using {:?} as config file.", &config_file);

    let config = fs::read_to_string(&config_file);
    if let Err(e) = config {
      println!("Error reading config: {}", e);
      return;
    }
    let config = config.expect("Already checked Err case");
    let config = toml::from_str::<config::Config>(&config);
    if let Err(e) = config {
      println!("Error parsing config: {}", e);
      return;
    }
    let config = config.expect("Already checked Err case");

    // For debugging set DUMP_CONFIG=1 or DUMP_CONFIG=true
    if let Ok(val) = env::var("DUMP_CONFIG") {
      if val.contains("1") || val.contains("t") {
        println!("config={:#?}", &config);
      }
    }




}


fn get_first_path(paths: &[&str]) -> Result<PathBuf, ()> {
  for p in paths {
    if Path::new(p).exists() {
      return Ok(PathBuf::from(p));
    }
  }
  Err(())
}


