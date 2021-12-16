use crate::method::Method;

pub struct AppArgument {
  pub address: String,
  pub method: Method,
  pub body: Option<String>,
}

impl Default for AppArgument {
  fn default() -> Self {
    AppArgument {
      address: String::from("127.0.0.1"),
      method: Method::default(),
      body: None,
    }
  }
}
