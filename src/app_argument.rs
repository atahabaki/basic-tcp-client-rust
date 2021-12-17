use crate::{address::Address, method::Method};

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

impl ToString for AppArgument {
  fn to_string(&self) -> String {
    let l1 = format!(
      "{} /{} HTTP/1.1",
      self.method.to_string(),
      self.address.path.as_ref().unwrap_or(&String::new())
    );
    let l2 = format!(
      "Host: {}:{}",
      self.address.host,
      self.address.port.as_ref().unwrap_or(&String::from("80"))
    );
    let l3 = format!("{}", self.body.as_ref().unwrap_or(&String::new()));
    let l = if self.method.to_string() == Method::GET.to_string() {
      format!("{}\n{}\r\n\r\n{}\r\n", l1, l2, l3)
    } else {
      let l5 = format!(
        "Content-Length: {}",
        self.body.as_ref().unwrap_or(&String::new()).len() + 1
      );
      let l6 = "Content-Type: application/json";
      format!("{}\r\n{}\r\n{}\r\n{}\r\n\r\n{}\r\n", l1, l2, l6, l5, l3)
    };
    dbg!("{:#?}", &l);
    l
  }
}
