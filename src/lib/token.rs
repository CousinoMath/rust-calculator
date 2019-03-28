//! Lexical tokens used by the calculator.

use std::fmt;

/// An enumeration for the tokens accepted by the calculator.
#[derive(Clone, PartialEq, Debug)]
pub enum Token {
  LParen,
  RParen,
  Plus,
  Minus,
  Star,
  Slash,
  Caret,
  Equals,
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
      Token::Equals => write!(f, "="),
      Token::Number(num) => write!(f, "{}", num),
      Token::Eoi => write!(f, "♣"),
      Token::Constant(name) => write!(f, "{}", name),
      Token::Function(name) => write!(f, "{}", name),
      Token::Identifier(name) => write!(f, "{}", name),
    }
  }
}

/// A helper function to distinguish various kinds of identifiers: variables,
/// constancts, and functions
///
/// # Examples
///
/// ```
/// assert_eq!(recognize_identifier("pi"), Token::Constant("pi".to_string()));
/// assert_eq!(recognize_identifier("sqrt"), Token::Function("sqrt".to_string()));
/// assert_eq!(recognize_identifier("variable"), Token::Variable("variable".to_string()));
/// ```
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
