extern crate alloc;
use alloc::{
  format,
  string::{String, ToString},
  vec::Vec,
};
use noli::net::{SocketAddr, TcpStream, lookup_host};
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
    let ips = match lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => return Err(Error::Network(format!("Failed to find IP address: {:#?}", e))),
    };

    if ips.len() < 1 {
      return Err(Error::Network("Failed to find IP address".to_string()));
    }

    let socket_add: SocketAddr = (ips[0], port).into();

    let mut stream = match TcpStream::connect(socket_add) {
      Ok(stream) => stream,
      Err(_) => return Err(Error::Network("Failed to connect to TCP stream".to_string())),
    };

    let mut request = String::new();
    request.push_str(&format!("GET /{path} HTTP/1.1\n"));
    request.push_str(&format!("Host: {host}\n"));
    request.push_str("Accept: text/html\n");
    request.push_str("Connection: close\n");
    request.push_str("\n");

    let _bytes_written = match stream.write(request.as_bytes()) {
      Ok(bytes) => bytes,
      Err(_) => return Err(Error::Network("Failed to send a request to TCP stream".to_string())),
    };

    let mut recieved = Vec::new();
    loop {
      let mut buf = [0u8; 4096];
      let bytes_read = match stream.read(&mut buf) {
        Ok(bytes) => bytes,
        Err(_) => {
          return Err(Error::Network(
            "Failed to receive a request from TCP stream".to_string(),
          ));
        }
      };
      if bytes_read == 0 {
        break;
      }
      recieved.extend_from_slice(&buf[..bytes_read]);
    }

    match core::str::from_utf8(&recieved) {
      Ok(response) => HttpResponse::new(response.to_string()),
      Err(e) => return Err(Error::Network(format!("Failed to parse response: {:#?}", e))),
    }
  }
}
