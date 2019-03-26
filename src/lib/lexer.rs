use crate::lib::token::Token;

pub struct Lexer {
  current_start: usize,
  current_end: usize,
  current: char,
  source: String, 
}

impl Lexer {
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
      Err(messages.iter().fold("".to_string(), |acc, x| acc + x))
    }
  }

  fn new(source: &str) -> Lexer {
    let current = source.chars().next();
    Lexer {
      current_start: 0,
      current_end: current.map_or(0, |c| c.len_utf8()),
      current: current.unwrap_or('\0'),
      source: source.to_string(),
    }
  }

  fn next_token(&mut self) -> Result<Token, String> {
    self.skip_whitespace();
    match self.current {
      '(' => Ok(Token::LParen),
      ')' => Ok(Token::RParen),
      '+' => Ok(Token::Plus),
      '-' => Ok(Token::Minus),
      '*' => Ok(Token::Star),
      '/' => Ok(Token::Slash),
      '^' => Ok(Token::Caret),
      c if c.is_ascii_digit() || c == '.' =>
          self.parse_number().map(|n| Token::Number(n)),
      _ => Err(format!("Unrecognized character {}", self.current)),
    }
  }

  fn advance(&mut self) {
    self.current_start = self.current_end;
    if self.current_start < self.source.len() {
      self.current = self.source[self.current_start..].chars().next().unwrap_or('\0');
      self.current_end += self.current.len_utf8();
    }
  }

  fn skip_whitespace(&mut self) {
    while self.current.is_whitespace() {
      self.advance();
    }
  }

  fn parse_number(&mut self) -> Result<f64, String> {
    let mut numeric_chars = Vec::new();
    while self.current.is_ascii_digit() || self.current == '.' {
      numeric_chars.push(self.current);
    }
    let numeric_string = numeric_chars.iter().collect::<String>();
    match numeric_string.parse::<f64>() {
      Ok(number) => Ok(number),
      Err(_) => Err(format!("Failed to parse number from '{}'", numeric_string)),
    }
  }
}