//! Lexical analyzer for the calculator

use crate::lib::token::{recognize_identifier, Token};
use crate::lib::unlines;

/// Lexer state
pub struct Lexer {
  /// The beginning index of a token's first code point in the source string
  initial: usize,
  /// The starting index of the current code point in the source string
  current_start: usize,
  /// The starting index of the *next* code point in the source string
  current_end: usize,
  /// The current code point
  current: char,
  /// The source string
  source: String,
}

impl Lexer {
  /// Lexes a given string and returns a result.
  /// 
  /// # Examples
  /// 
  /// ```
  /// assert_eq!(Lexer::lex("("), Ok(Token::LParen));
  /// assert_eq!(Lexer::lex("2.71828182845904523536"), Ok(Token::Number(2.71828182845904523536)));
  /// assert!(Lexer::lex("0.1.0").is_err());
  /// ```
  pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    let mut messages = Vec::new();
    while lexer.current_start < lexer.current_end {
      match lexer.next_token() {
        Ok(token) => tokens.push(token),
        Err(message) => messages.push(message),
      }
    }
    if messages.len() == 0 {
      tokens.push(Token::Eoi);
      Ok(tokens)
    } else {
      Err(unlines(messages).trim().to_string())
    }
  }

  /// Create a new lexer from a source string.
  fn new(source: &str) -> Lexer {
    let current = source.chars().next();
    Lexer {
      initial: 0,
      current_start: 0,
      current_end: current.map_or(0, |c| c.len_utf8()),
      current: current.unwrap_or('\0'),
      source: source.to_string(),
    }
  }

  /// Find the next token in the source string.
  fn next_token(&mut self) -> Result<Token, String> {
    self.skip_whitespace();
    self.initial = self.current_start;
    match self.current {
      '(' => {
        self.advance();
        Ok(Token::LParen)
      }
      ')' => {
        self.advance();
        Ok(Token::RParen)
      }
      '+' => {
        self.advance();
        Ok(Token::Plus)
      }
      '-' => {
        self.advance();
        Ok(Token::Minus)
      }
      '*' => {
        self.advance();
        Ok(Token::Star)
      }
      '/' => {
        self.advance();
        Ok(Token::Slash)
      }
      '^' => {
        self.advance();
        Ok(Token::Caret)
      }
      '=' => {
        self.advance();
        Ok(Token::Equals)
      }
      c if c.is_ascii_digit() || c == '.' => self.lex_number().map(|n| Token::Number(n)),
      c if c.is_alphabetic() => self.lex_identifier().map(|id| recognize_identifier(&id)),
      c => {
        self.advance();
        Err(format!("Unrecognized character {}", c))
      }
    }
  }

  /// Advance the lexer by one code point in the source string.
  fn advance(&mut self) {
    self.current_start = self.current_end;
    if self.current_start < self.source.len() {
      self.current = self.source[self.current_start..]
        .chars()
        .next()
        .unwrap_or('\0');
      self.current_end += self.current.len_utf8();
    } else {
      self.current = '\0';
    }
  }

  /// Skips over whitespace in the source string.
  fn skip_whitespace(&mut self) {
    while self.current.is_whitespace() {
      self.advance();
    }
  }

  /// Determines whether the end of input has been reached.
  fn hit_eoi(&self) -> bool {
    self.current_start >= self.current_end
  }

  /// Lexes and parses a number into a `f64` float.
  fn lex_number(&mut self) -> Result<f64, String> {
    let mut numeric_chars: Vec<char> = Vec::new();
    while (self.current.is_ascii_digit() || self.current == '.') && !self.hit_eoi() {
      numeric_chars.push(self.current);
      self.advance();
    }
    let numeric_string = numeric_chars.iter().collect::<String>();
    numeric_string
      .parse::<f64>()
      .map_err(|_| format!("Failed to parse '{}' as a number.", numeric_string))
  }

  /// Lexes an identifier
  fn lex_identifier(&mut self) -> Result<String, String> {
    let mut chars: Vec<char> = Vec::new();
    if self.current.is_alphabetic() {
      chars.push(self.current);
      self.advance();
    }
    while self.current.is_alphanumeric() && !self.hit_eoi() {
      chars.push(self.current);
      self.advance();
    }
    let identifier = chars.iter().collect::<String>();
    if identifier.len() > 0 {
      Ok(identifier)
    } else {
      Err("Identifiers must be alphanumeric and start with an alphabetic character.".to_string())
    }
  }
}

#[cfg(test)]
mod test {
  use crate::lib::lexer::Lexer;
  use crate::lib::lexer::Token;

  #[test]
  fn test_parse_number() {
    let tokens = Lexer::lex("2.71828182845904523536");
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    let mut tokens = tokens.iter();
    assert_eq!(
      tokens.next().unwrap(),
      &Token::Number(2.71828182845904523536)
    );
  }

  #[test]
  fn test_parse_symbols() {
    let token = Lexer::lex("(");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::LParen);

    let token = Lexer::lex(")");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::RParen);

    let token = Lexer::lex("+");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::Plus);

    let token = Lexer::lex("-");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::Minus);

    let token = Lexer::lex("*");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::Star);

    let token = Lexer::lex("/");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::Slash);

    let token = Lexer::lex("^");
    assert!(token.is_ok());
    let token = token.unwrap();
    let mut token = token.iter();
    assert_eq!(token.next().unwrap(), &Token::Caret);

    let token = Lexer::lex("&");
    assert!(token.is_err());
  }
}
