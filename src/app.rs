use std::{
  io::{Read, Write},
  net::TcpStream,
  process::exit,
};

use crate::{address::Address, app_argument::AppArgument, method::Method};

pub struct App {
  pub args: AppArgument,
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
        app_args.address = Address::from_string(args[1].clone());
        Ok(app_args)
      }
      3 => {
        //exec <address> <method> : Probably POST with no body?...
        let mut app_args = AppArgument::default();
        app_args.address = Address::from_string(args[1].clone());
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
        app_args.address = Address::from_string(args[1].clone());
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

  pub fn new(args: Vec<String>) -> Self {
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

  pub fn send_req(&self) {
    let addr = format!(
      "{}:{}",
      self.args.address.host,
      self
        .args
        .address
        .port
        .as_ref()
        .unwrap_or(&String::from("80"))
    );
    match TcpStream::connect(&addr) {
      Ok(mut stream) => {
        let buf = self.args.to_string();
        match &mut stream.write_all(buf.as_bytes()) {
          Ok(_) => {}
          Err(e) => panic!("{}", e),
        };
        let mut buffer = String::new();
        loop {
          match stream.read_to_string(&mut buffer) {
            Ok(status) => {
              // means OK, everything went right, close the program!..
              if status == 0 {
                exit(0);
              } else {
                println!("{}", buffer);
                exit(0);
              }
            }
            Err(e) => {
              eprintln!(
                "Something went wrong here while trying to read response from {}",
                addr
              );
              panic!("{}", e);
            }
          }
        }
      }
      Err(e) => {
        panic!("{}", e);
      }
    }
  }
}
