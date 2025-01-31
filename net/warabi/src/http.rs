#![no_std]

pub mod http;

pub struct HttpClient {}

extern crate alloc;
use alloc::string::String;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse,Error> {
    //あとで
  }
}
