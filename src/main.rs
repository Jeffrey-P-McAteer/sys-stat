
use dirs;
use toml;
use humantime;
use ctrlc;

use std::path::{
  Path, PathBuf
};
use std::fs;
use std::env;
use std::time::{
  SystemTime, UNIX_EPOCH, Duration
};
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

mod config;
mod check;

#[cfg(feature = "web-view")]
mod gui;

fn main() {
    // a signal handler can change this to tell the loop at
    // the end to exit gracefully. When exit_flag == true
    // an exit is requested.
    let exit_flag = Arc::new(AtomicBool::new(false));
    let ef = exit_flag.clone();

    let e = ctrlc::set_handler(move || {
      ef.store(true, Ordering::SeqCst);
    });
    if let Err(e) = e {
      println!("Error setting exit handler: {}", e);
    }

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
    let config = config.expect("Already checked Err case");

    // For debugging set DUMP_CONFIG=1 or DUMP_CONFIG=true
    if let Ok(val) = env::var("DUMP_CONFIG") {
      if val.contains("1") || val.contains("t") {
        println!("config={:#?}", &config);
      }
    }

    // We clone this because of mutability rules related to config.sys in the main loop below
    let general = config.general.clone();

    
    // This fn changes depending on the "web-view" feature.
    // See the conditionally compiled functions below.
    main_loop(Arc::new(Mutex::new(config)), general, exit_flag);
}

// GUI main function
#[cfg(feature = "web-view")]
fn main_loop(mut config: Arc<Mutex<config::Config>>, general: config::General, exit_flag: Arc<AtomicBool>) {
  // Copy/clone references so we can pass them to the BG thread and use them on the main thread
  let ef = exit_flag.clone();

  // Run check loop in background
  let check_config = config.clone();
  let th = thread::spawn(move || {
    system_check_loop(check_config, general, exit_flag);
  });
  
  // Run graphics on main thread (win32 is picky about this)
  if let Err(e) = gui::blocking_gui_main(config.clone(), ef.clone()) {
    println!("Graphics error: {}", e);
  }

  ef.store(true, Ordering::SeqCst);
  println!("Waiting for background thread to exit...");
  th.join();
}

// CLI main function
#[cfg(not(feature = "web-view"))]
fn main_loop(mut config: Arc<Mutex<config::Config>>, general: config::General, exit_flag: Arc<AtomicBool>) {
  system_check_loop(config, general, exit_flag);
}


fn system_check_loop(mut config: Arc<Mutex<config::Config>>, general: config::General, exit_flag: Arc<AtomicBool>) {
  // Infinite loop which keeps track of system status and appends
  // latency information to config.log_file
  // We sleep in chunks to respond quickly when exit_flag changes.
  let delay_chunk = time::Duration::from_millis(50);
  let delay_count = 2_000 / 50; // 2_000ms = chunk * delay_count
  'main_loop: loop {

    if let Ok(mut config) = config.lock() {
      for i in 0..(&config).sys.len() {
        let sys = &mut config.sys[i];
        if needs_check(sys) {
          check::check(&general, sys);
          update_last_check_time(sys);
        }
        if exit_flag.load(Ordering::SeqCst) {
          break 'main_loop;
        }
      }
    }
    
    for _ in 0..delay_count {
      thread::sleep(delay_chunk);
      if exit_flag.load(Ordering::SeqCst) {
        break 'main_loop;
      }
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

