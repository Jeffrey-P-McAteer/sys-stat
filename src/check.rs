
use reqwest;
use humantime;

use crate::config;
use crate::append;

use std::time::{SystemTime, Duration};

pub fn check(general: &config::General, sys: &config::System) {
  println!("Checking '{}'...", sys.name);
  let timeout = humantime::parse_duration(&sys.response_must_finish_within);
  if let Err(e) = timeout {
    println!("Error parsing 'response_must_finish_within' for '{}': {} ({})", sys.name, sys.response_must_finish_within, e);
    return;
  }
  
  let timeout = timeout.expect("Checked err case");
  let begin = SystemTime::now();
  let success: bool;

  match http_req(&sys.uri, timeout) {
    Ok(response_txt) => {
      //println!("got {}", &response_txt);
      if let Some(req_content_text) = sys.response_must_contain.clone() {
        if response_txt.contains(&req_content_text) {
          success = true;
        }
        else {
          success = false;
        }
      }
      else {
        success = true;
      }
    }
    Err(e) => {
      println!("error making request: {:?}", &e);
      success = false;
    }
  }

  let end = SystemTime::now();
  let duration_ms = end.duration_since(begin).expect("Time went backwards, oh no!").as_millis();

  // Log timestamp, system name, success, and duration to general.log_file
  append(&general.log_file, format!(
    "{}, {}, {}, {}",
    humantime::format_rfc3339(end), sys.name, success, duration_ms
  ));

}

fn http_req(url: &str, timeout: Duration) -> Result<String, Box<dyn std::error::Error>> {
  let client = reqwest::blocking::Client::builder()
    .timeout(timeout)
    .danger_accept_invalid_certs(true)
    .build()?;
  let resp = client.get(url).send()?.text().unwrap_or(String::new());
  Ok(resp)
}

