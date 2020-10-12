
use serde::{
  Deserialize
};

#[derive(Debug, Deserialize)]
pub struct Config {
    general: General,
    sys: Vec<System>,
}

#[derive(Debug, Deserialize)]
pub struct General {
  log_file: String,
  on_status_change: Vec<String>,
  on_status_good: Vec<String>,
  on_status_bad: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct System {
  name: String,
  uri: String,
  
  #[serde(default = "default_description")]
  description: String,
  response_must_contain: Option<String>,
  #[serde(default = "default_response_must_finish_within")]
  response_must_finish_within: String,
  #[serde(default = "default_check_interval")]
  check_interval: String,
}


fn default_description() -> String {
  return "".to_string();
}

fn default_response_must_finish_within() -> String {
  return "60s".to_string();
}

fn default_check_interval() -> String {
  return "10m".to_string();
}

