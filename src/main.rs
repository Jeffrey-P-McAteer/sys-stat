
use dirs;
use toml;
use humantime;

use std::path::{
  Path, PathBuf
};
use std::fs;
use std::env;
use std::time::{
  SystemTime, UNIX_EPOCH, Duration
};
use std::{thread, time};

mod config;
mod check;

fn main() {
    let home_dir = dirs::home_dir().unwrap_or(PathBuf::from("."));
    // The first file in this list which exists is picked
    // as the config file
    let possible_config_files = [
      "target/sys-stat.toml", // This is a .gitignore-d directory  we can use for quickly testing endpoints not publicly tracked
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
    let config_file = fs::canonicalize(&config_file).expect("Could not get absolute path of file"); // very rare
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
    let mut config = config.expect("Already checked Err case");

    // For debugging set DUMP_CONFIG=1 or DUMP_CONFIG=true
    if let Ok(val) = env::var("DUMP_CONFIG") {
      if val.contains("1") || val.contains("t") {
        println!("config={:#?}", &config);
      }
    }

    // We clone this because of mutability rules related to config.sys
    let general = config.general.clone();

    // Infinite loop which keeps track of system status and appends
    // latency information to config.log_file
    let delay = time::Duration::from_millis(2_000);
    loop {
      //for &mut sys in &config.sys {
      for i in 0..config.sys.len() {
        let sys = &mut config.sys[i];
        if needs_check(sys) {
          check::check(&general, sys);
          update_last_check_time(sys);
        }
      }
      thread::sleep(delay);
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

fn needs_check(sys: &config::System) -> bool {
  let duration = humantime::parse_duration(&sys.check_interval);
  if let Err(e) = duration {
    println!("Error: check_interval for system '{}' is invalid: {} ({})", sys.name, sys.check_interval, e);
    return false;
  }
  let duration = duration.expect("Checked Err case");
  let next_check_time = UNIX_EPOCH + Duration::from_secs( sys.last_check_epoch_seconds ) + duration;
  return SystemTime::now() > next_check_time;
}

fn update_last_check_time(sys: &mut config::System) {
  sys.last_check_epoch_seconds = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
}

fn append(path: &str, line: String) {
  use std::fs::OpenOptions;
  use std::io::prelude::*;

  let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("Coult not open log file to append a new line");

  if let Err(e) = writeln!(file, "{}", line) {
    println!("Error writing to {}: {}", path, e);
  }

}

