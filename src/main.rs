use std::{env, process::exit};

use app::App;
use app_argument::AppArgument;
use method::Method;

mod app;
mod method;
mod app_argument;

fn main() {
  let args: Vec<String> = env::args().collect();
  let app = App::new(args);
  app.send_req();
}
