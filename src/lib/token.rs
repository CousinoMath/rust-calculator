use std::fmt;

#[derive(Clone, Copy)]
pub enum Token {
  LParen,
  RParen,
  Plus,
  Minus,
  Star,
  Slash,
  Caret,
  Number(f64),
  Eoi,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Token::LParen => write!(f, "("),
      Token::RParen => write!(f, ")"),
      Token::Plus => write!(f, "+"),
      Token::Minus => write!(f, "-"),
      Token::Star => write!(f, "*"),
      Token::Slash => write!(f, "/"),
      Token::Caret => write!(f, "^"),
      Token::Number(num) => write!(f, "{}", num),
      Token::Eoi => write!(f, ""),
    }
  }
}

pub fn is_eoi(token: Token) -> bool {
  match token {
    Token::Eoi => true,
    _ => false,
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_is_eoi() {
    use crate::lib::token::Token;
    use crate::lib::token::is_eoi;
    assert!(is_eoi(Token::Eoi));
    assert!(!is_eoi(Token::LParen)); 
  }
}