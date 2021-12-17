#[derive(PartialEq)]
pub enum Method {
  GET,
  POST,
}

impl Method {
  pub fn from_string(method: String) -> Option<Method> {
    match &method.trim().to_lowercase()[..] {
      "get" => Some(Method::GET),
      "post" => Some(Method::POST),
      _ => None,
    }
  }
}

impl ToString for Method {
  fn to_string(&self) -> String {
    match self {
      Method::GET => String::from("GET"),
      Method::POST => String::from("POST"),
    }
  }
}

impl Default for Method {
  fn default() -> Self {
    Method::GET
  }
}
