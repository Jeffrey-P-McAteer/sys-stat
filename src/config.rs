
use serde::{
  Deserialize
};

#[derive(Debug, Deserialize)]
pub struct Config {
  pub general: General,
  pub sys: Vec<System>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
  pub log_file: String,
  pub on_status_change: Vec<String>,
  pub on_status_good: Vec<String>,
  pub on_status_bad: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct System {
  pub name: String,
  pub uri: String,
  
  #[serde(default = "default_description")]
  pub description: String,
  pub response_must_contain: Option<String>,
  #[serde(default = "default_response_must_finish_within")]
  pub response_must_finish_within: String,
  #[serde(default = "default_check_interval")]
  pub check_interval: String,
  #[serde(default = "default_last_check_epoch_seconds")]
  pub last_check_epoch_seconds: u64,
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

fn default_last_check_epoch_seconds() -> u64 {
  return 0;
}

