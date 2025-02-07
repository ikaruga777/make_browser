use alloc::string::String;
use alloc::vec::Vec;
use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
  StartTag {
    tag: String,
    self_closing: bool,
    attributes: Vec<Attribute>,
  },
  EndTag {
    tag: String,
  },
  Char(char),
  Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
  Data,
  TagOpen,
  EndTagOpen,
  TagName,
  BeforeAttributeName,
  AttributeName,
  AfterAttributeName,
  BeforeAttributeValue,
  AttributeValueDoubleQuoted,
  AttributeValueSingleQuoted,
  AfterAttributeValueQuoted,
  SelfClosingStartTag,
  ScriptData,
  ScriptDataLessThanSign,
  ScriptDataEndTagOpen,
  ScriptDataEndTagName,
  TemporaryBuffer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
  state: State,
  pos: usize,
  reconsume: bool,
  latest_token: Option<HtmlToken>,
  input: Vec<char>,
  buf: String,
}

impl HtmlTokenizer {
  pub fn new(html: String) -> Self {
    Self {
      state: State::Data,
      pos: 0,
      reconsume: false,
      latest_token: None,
      input: html.chars().collect(),
      buf: String::new(),
    }
  }
}

impl Iterator for HtmlTokenizer {
  type Item = HtmlToken;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos >=self.input.len() {
      return None;
    }

    loop {
      let c = self.consume_next_input();
      match self.state {
        State::Data => {
          if c == '<' {
            self.state = State::TagOpen;
            continue;
          }

          if self.is_eof() {
            return Some(HtmlToken::Eof);
          }

          return Some(HtmlToken::Char(c));
        }
        _ => {}
      }
    }
  }

  fn consume_next_input(&mut self) -> char {
    let c = self.input[self.pos];
    self.pos += 1;
    c
  }
}
