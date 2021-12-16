use std::{env, process::exit};

enum Method {
  GET,
  POST,
}

impl Method {
  fn from_string(method: String) -> Option<Method> {
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

struct AppArgument {
  address: String,
  method: Method,
  body: Option<String>,
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

struct App {
  args: AppArgument,
}

impl App {
  fn _validate_args(args: Vec<String>) -> Result<AppArgument, &'static str> {
    match args.len() {
      1 => {
        // Use defaults...
        Ok(AppArgument::default())
      }
      2 => {
        //exec <address> : Use defaults...
        let mut app_args = AppArgument::default();
        app_args.address = args[1].clone();
        Ok(app_args)
      }
      3 => {
        //exec <address> <method> : Probably POST with no body?...
        let mut app_args = AppArgument::default();
        app_args.address = args[1].clone();
        if let Some(method) = Method::from_string(args[2].clone()) {
          app_args.method = method;
        } else {
          panic!("Failed to parse method!..");
        }
        Ok(app_args)
      }
      4 => {
        //exec <address> <method> <body> : Probably POST...
        let mut app_args = AppArgument::default();
        app_args.address = args[1].clone();
        if let Some(method) = Method::from_string(args[2].clone()) {
          app_args.method = method;
        } else {
          panic!("Failed to parse method!..");
        }
        app_args.body = Some(args[3].clone());
        Ok(app_args)
      }
      _ => Err("Missing arguments or arguments exceeded the limit..."),
    }
  }

  fn new(args: Vec<String>) -> Self {
    match Self::_validate_args(args) {
      Ok(app_args) => App { args: app_args },
      Err(e) => {
        Self::usage(Some("Failed to parse arguments!.."));
        panic!("{}", e);
      }
    }
  }

  fn usage(msg: Option<&str>) {
    if let Some(m) = msg {
      eprintln!("{}", m);
    }
    println!("USAGE:\ntcpc <address> [method] [body]\nAvailable methods are: GET and POST");
    println!("You could leave GET's body empty, but don't leave empty the POST's body...");
  }

  fn send_req(&self) {
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let app = App::new(args);
  app.send_req();
}
