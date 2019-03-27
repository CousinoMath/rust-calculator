use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
  LParen,
  RParen,
  Plus,
  Minus,
  Star,
  Slash,
  Caret,
  Number(f64),
  Identifier(String),
  Constant(String),
  Function(String),
  Eoi,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.clone() {
      Token::LParen => write!(f, "("),
      Token::RParen => write!(f, ")"),
      Token::Plus => write!(f, "+"),
      Token::Minus => write!(f, "-"),
      Token::Star => write!(f, "*"),
      Token::Slash => write!(f, "/"),
      Token::Caret => write!(f, "^"),
      Token::Number(num) => write!(f, "{}", num),
      Token::Eoi => write!(f, "♣"),
      Token::Constant(name) => write!(f, "{}", name),
      Token::Function(name) => write!(f, "{}", name),
      Token::Identifier(name) => write!(f, "{}", name),
    }
  }
}

pub fn is_eoi(token: Token) -> bool {
  match token {
    Token::Eoi => true,
    _ => false,
  }
}

pub fn recognize_identifier(identifier: &str) -> Token {
  let constants = ["e", "pi", "π"];
  let functions = [
    "abs", "acos", "acosh", "asin", "asinh", "atan", "atanh", "cos", "cosh", "exp", "log", "sin",
    "sinh", "sqrt", "tan", "tanh",
  ];
  if let Ok(index) = constants.binary_search(&identifier) {
    Token::Constant(constants[index].to_owned())
  } else if let Ok(index) = functions.binary_search(&identifier) {
    Token::Function(functions[index].to_owned())
  } else {
    Token::Identifier(identifier.to_owned())
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_is_eoi() {
    use crate::lib::token::is_eoi;
    use crate::lib::token::Token;
    assert!(is_eoi(Token::Eoi));
    assert!(!is_eoi(Token::LParen));
  }
}
