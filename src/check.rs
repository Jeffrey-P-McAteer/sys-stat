
use reqwest;
use humantime;
use handlebars;
use serde_json::json;

use crate::config;
use crate::append;

use std::time::{SystemTime, Duration};
use std::process::Command;

pub fn check(general: &config::General, sys: &mut config::System) {
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

  if success {
    sys.status_history.push(config::Status::Good);
  }
  else {
    sys.status_history.push(config::Status::Bad);
  }
  while sys.status_history.len() as u64 > general.status_history_length {
    sys.status_history.pop();
  }

  // Also run various tasks from [general] based on the system status change

  // We require the status to be the same for all general.status_history_length,
  // which adds a small delay but reduces false positives for smaller local-only
  // errors (eg wifi signal is low which causes a disconnect).
  // To use immediate results, simply set general.status_history_length = 1
  // in your config file.
  let mut all_statuses_are_same = true;
  let mut status = config::Status::Bad;
  for s in &sys.status_history {
    if s != &status {
      all_statuses_are_same = false;
    }
  }

  // We only fire actions when the same thing is observed N times to prevent false positives.
  if ! all_statuses_are_same || sys.status_history.len() < 1 {
    return;
  }

  status = sys.status_history.get(0).unwrap_or(&config::Status::Bad).clone();

  let mut run_status_change = true;

  if let Some(last_status) = &sys.last_stable_status {
    if &status == last_status {
      run_status_change = false;
    }
  }

  if run_status_change && general.on_status_change.len() > 0{
    
    let mut formatted_args = vec![];
    
    let hb = handlebars::Handlebars::new();

    for s in &general.on_status_change[1..] {
      let formatted = hb.render_template(
        s.as_str(),
        &json!({
          "name": sys.name,
          "status": format!("{:?}", status),
          "reason": "http(s) endpoint did not respond", // TODO
        })
      );
      match formatted {
        Ok(val) => { formatted_args.push(val); }
        Err(e) => {
          println!("Error formatting command argument '{}', {}", s, e);
          run_status_change = false;
          break;
        }
      }
    }

    if run_status_change { // check 2nd time, may have been format error with given command format strings
      let child = Command::new(general.on_status_change[0].as_str())
        .args(&formatted_args[..])
        .spawn();

      if let Err(e) = child {
        println!("Error spawning general.on_status_change: {} (cmd run was {:?})", e, &general.on_status_change);
      }
    }
  }

}

fn http_req(url: &str, timeout: Duration) -> Result<String, Box<dyn std::error::Error>> {
  let client = reqwest::blocking::Client::builder()
    .timeout(timeout)
    .danger_accept_invalid_certs(true)
    .build()?;
  let resp = client.get(url).send()?.text().unwrap_or(String::new());
  Ok(resp)
}

