use std::env;

use app::App;

mod address;
mod app;
mod app_argument;
mod method;

fn main() {
  let args: Vec<String> = env::args().collect();
  let app = App::new(args);
  app.send_req();
}
