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
    let path = self.address.path.as_ref();
    let port = self.address.port.as_ref();
    let body = self.body.as_ref();
    let host = &self.address.host;
    let mutual = format!(
      "
{} /{} HTTP/1.1\r
Host: {}:{}\r",
      self.method.to_string(),
      path.unwrap_or(&String::new()),
      host,
      port.unwrap_or(&String::from("80"))
    );
    return if self.method == Method::GET {
      mutual + "\n\r\n"
    } else {
      format!(
        "{}
Content-Length: {}\r
Content-Type: application/json\r
\r
{}
\r
\r\n",
        mutual,
        body.unwrap_or(&String::new()).len(),
        body.unwrap_or(&String::new())
      )
    };
  }
}
