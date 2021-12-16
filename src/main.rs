use std::env;

use app::App;

mod app;
mod method;
mod address;
mod app_argument;

fn main() {
  let args: Vec<String> = env::args().collect();
  let app = App::new(args);
  app.send_req();
}
