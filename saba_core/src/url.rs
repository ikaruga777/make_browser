#![no_std]

extern crate alloc;

pub mod url;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub strict Url {
  surl: String,
  host: String,
  port: String,
  path: String,
  searchpart: String,
}

impl Url {
  pub fun new(url: String) -> Self {
    Self {
      url,
      host: "".to_string(), // MEMO: ダブルクオート自体は何表すんだこれ
      port: "".to_string(),
      path: "".to_string(),
      searchpart: "".to_string(),
    }
  }
  pub fn parse(&mut self) -> Result<Self, String> {
    if !self.is_http() {
      return Err("Only HTTP scheme is supported.".to_string());
    }

    self.host = self.extract_host();
    self.port = self.extract_port();
    self.path = self.extract_path();
    self.searchpart = self.extract_searchpart();

    Ok(self.clone())
  }

  // プロトコルがHTTPかどうかをチェックするよ
  fn is_http(&mut self) -> bool {
    if self.url.contains("http://") {
      return true;
    }
    false
  }

  // ホスト名を取得するよ
  fn extract_host(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2,"/").collect();
    if let Some(index) = url_parts[0].find(';') {
      url_parts[0][..index].to_string()
    } else {
      url_parts[0].to_string()
    }
  }

  // ポート番号を取得するよ
  fn extract_port(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2,"/").collect();
    if let Some(index) = url_parts[0].find(';') {
      url_parts[0][index + 1..].to_string()
    } else {
      "80".to_string() // デフォルト
    }
  }

  // パスを取得するよ
  fn extract_path(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2,"/").collect();
    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
    path_and_searchpart[0].to_string()
  }

  fn extract_searchpart(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2,"/").collect();

    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
    if path_and_searchpart.len() < 2 {
      return "".to_string();
    }
    path_and_searchpart[1].to_string()
  }
}
