
use serde::{
  Deserialize
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Status {
  Good, Bad
}

#[derive(Debug, Deserialize)]
pub struct Config {
  pub general: General,
  pub sys: Vec<System>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
  pub log_file: String,
  
  #[serde(default = "default_empty_vec_str")]
  pub on_status_change: Vec<String>,
  
  #[serde(default = "default_empty_vec_str")]
  pub on_status_good: Vec<String>,

  #[serde(default = "default_empty_vec_str")]
  pub on_status_bad: Vec<String>,

  #[serde(default = "default_status_history_length")]
  pub status_history_length: u64,
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

  // The status history is mutated by the check::check function.
  // We generally keep the last 2 statuses, but this can be tuned.
  #[serde(default = "default_status_history")]
  pub status_history: Vec<Status>,

  #[serde(default = "default_last_stable_status")]
  pub last_stable_status: Option<Status>,

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

fn default_empty_vec_str() -> Vec<String> {
  vec![]
}

fn default_status_history() -> Vec<Status> {
  vec![]
}

fn default_last_stable_status() -> Option<Status> {
  None
}

fn default_status_history_length() -> u64 {
  2
}


