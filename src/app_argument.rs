use crate::{method::Method, address::Address};

pub struct AppArgument {
  pub address: Address,
  pub method: Method,
  pub body: Option<String>,
}

impl Default for AppArgument {
  fn default() -> Self {
    AppArgument {
      address: Address::default(),
      method: Method::default(),
      body: None,
    }
  }
}