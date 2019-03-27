use crate::lib::token::{ Token, recognize_identifier, };
use crate::lib::unlines;

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
      Err(unlines(messages).trim().to_string())
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
      c if c.is_ascii_digit() || c == '.' => self.lex_number().map(|n| Token::Number(n)),
      c if c.is_alphabetic() => self.lex_identifier().map(|id| recognize_identifier(&id)),
      c => {
        self.advance();
        Err(format!("Unrecognized character {}", c))
      }
    }
  }

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

  fn skip_whitespace(&mut self) {
    while self.current.is_whitespace() {
      self.advance();
    }
  }

  fn hit_eoi(&self) -> bool {
    self.current_start >= self.current_end
  }

  fn lex_number(&mut self) -> Result<f64, String> {
    let mut numeric_chars: Vec<char> = Vec::new();
    while self.current.is_ascii_digit() && !self.hit_eoi() {
      numeric_chars.push(self.current);
      self.advance();
    }
    if self.current == '.' {
      numeric_chars.push(self.current);
      self.advance();
      while self.current.is_ascii_digit() && !self.hit_eoi() {
        numeric_chars.push(self.current);
        self.advance();
      }
    }
    let numeric_string = numeric_chars.iter().collect::<String>();
    numeric_string
      .parse::<f64>()
      .map_err(|_| format!("Failed to parse '{}'", numeric_string))
  }

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
    Ok(chars.iter().collect::<String>())
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
