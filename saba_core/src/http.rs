#![no_std]

extern crate alloc;

pub mod http;
pub mod url;

use alloc::format;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct HttpResponse {
  version: String,
  status_code: u32,
  reason: String,
  headers: Vec<Header>,
  body: String,
}

#[derive(Debug, Clone)]
pub struct Header {
  name: String,
  value: String,
}

impl Header {
  pub fn new(name: String, value: String) -> Self {
    Self { name, value }
  }
}

impl HttpResponse {
  pub fn new(raw_response: String) -> Result<Self, Error> {
    let preprocessed_response = raw_response.trim_start().replace("\r\n", "\n");

    let (status_line, remaining) = match preprocessed_response.split_once("\n") {
      Some((s,r)) => (s, r),
      None => {
        return Err(Error::Network(format!("Invalid http response: {}", preprocessed_response)))
      }
    };
  }
}
