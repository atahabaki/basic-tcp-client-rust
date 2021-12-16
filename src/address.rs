#[derive(Debug)]
pub struct Address {
  pub host: String,
  pub port: Option<String>,
  pub path: Option<String>,
}

impl Address {
  pub fn from_string(address: String) -> Self {
    enum LoopState {
      Host,
      Port,
      Path,
    }
    let mut iter = address.chars().peekable();
    let mut host = String::new();
    let mut port = String::new();
    let mut path = String::new();
    let mut state = LoopState::Host;
    loop {
      if let Some(c) = iter.next() {
        if c == ':' {
          state = LoopState::Port;
          continue;
        } else if c == '/' {
          state = LoopState::Path;
          continue;
        } else {
          match state {
            LoopState::Host => {
              host.push(c);
            }
            LoopState::Port => {
              port.push(c);
            }
            LoopState::Path => {
              path.push(c);
            }
          }
        }
      } else {
        break;
      }
    }
    // Aslinda kontrol falan yapilmali ama peh :P asdf
    if host.len() <= 2 {
      panic!("Host's length at least should be bigger than 3..");
    }
    // Bence bu kadari kafi'...
    Self {
      host,
      port: if String::is_empty(&port) {
        None
      } else {
        Some(port)
      },
      path: if String::is_empty(&path) {
        None
      } else {
        Some(path)
      },
    }
  }
}

impl Default for Address {
  fn default() -> Self {
    Address {
      host: "127.0.0.1".into(),
      port: Some("80".into()),
      path: Some("/".into()),
    }
  }
}
