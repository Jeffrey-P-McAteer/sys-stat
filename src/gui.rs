
use web_view::*;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::config::Config;

pub fn blocking_gui_main(config: Arc<Mutex<Config>>, exit_flag: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
  
  let webview = web_view::builder()
      .title("Sys-Stat GUI")
      .content(Content::Html(GUI_HTML))
      .size(800, 600)
      .resizable(true)
      .debug(false)
      .user_data(config.clone())
      .invoke_handler(|webview, arg| {
          match arg {
              "reset" => {
                  // *webview.user_data_mut() += 10;
                  // let mut counter = counter.lock().unwrap();
                  // *counter = 0;
                  // render(webview, *counter)?;
              }
              "exit" => {
                  webview.exit();
              }
              _ => unimplemented!(),
          };
          Ok(())
      })
      .build()?;

  let handle = webview.handle();

  thread::spawn(move || loop {
    // Poll changes + push to GUI
    if exit_flag.load(Ordering::SeqCst) {
      handle.dispatch(|webview| {
        webview.exit();
        Ok(())
      }).expect("Fatal error dispatch exit task");
    }
    thread::sleep(Duration::from_millis(250));
  });

  webview.run()?;

  Ok(())
}

const GUI_HTML: &'static str = r#"<!DOCTYPE html>
<html>
  <head>

  </head>
  <body>
    <h1>Hello World</h1>
    <em>Hello gui</em>

  </body>
</html>
"#;

