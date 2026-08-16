#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
  let client = HttpClient::new();
  match client.get("host.test".to_string(), 3000, "".to_string()) {
    Ok(response) => {
      print!("response:\n{:#?}", response);
    }
    Err(err) => {
      print!("error:\n{:#?}", err);
    }
  }
  0
}

entry_point!(main);
