#![no_std]

pub mod http;

pub struct HttpClient {}

extern crate alloc;
use alloc::string::String;
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
  }
}
