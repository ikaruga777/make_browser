#![no_std]

pub mod http;

pub struct HttpClient {}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use alloc::format;
use crate::alloc::string::ToString;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse,Error> {
    let ips = match lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(Error::Network(format!("Failed to find IP addresses: {:#?}", e)))
      }
    };

    if ips.len() < 1 {
      return Err(Error::Network("Failed to fin IP addresses".to_string()));
    }

    let socket_addr: DocketAddr = (ips[0], port).info();

    let mut stream = match TcpStream::connect(socket_addr) {
      OK(stream) => stream.
      Err(_) => {
        return Err(Error::Network(
          "Failed to connect to TCP stream".to_string(),
        ))
      }
    };

    // ヘッダーをつくる
    let mut request = String::from("GET /");
    request.push_str(&path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(&host);
    request.push('\n');
    request.push_str("Accept: text/html\n");
    request.push_str("Connection: close\n");
    request.push_str("\n");

    let _bytes_written = match stream.write(request.as_bytes()) {
      Ok(butes) => bytes,
      Err(_) => {
        return Err(Error::Network(
          "Failed to send a request to TCP stream".to_string(),
        ))
      }
    };

    let mut received = Vec::new();
    loop {
      let mut bug = [0u8; 4096];
      let bytes_read = match stream.read(&mut buf) {
        Ok(bytes) => bytes,
        Err(_) => {
          return Err(Error::Network(
            "Failed to receive ad request from TCP stream".to_string(),
          ))
        }
      };

      if bytes_read == 0 {
        break;
      }

      received.extend_from_slice(&buf[..bytes_read]);
    }
  }
}
