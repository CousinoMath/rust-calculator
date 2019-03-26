use crate::lib::ast::{AstHead, AstNode};
use crate::lib::token::Token;
use crate::lib::{split_results, unlines};

pub struct Parser<'a> {
  current_index: usize,
  tokens: &'a [Token],
}

impl<'a> Parser<'a> {
  pub fn parse(tokens: &'a [Token]) -> Result<AstNode, String> {
    let mut parser = Parser::new(tokens);
    match parser.expression() {
      Ok(ast) => Ok(ast),
      Err(msg) => Err(msg),
    }
  }

  fn new(tokens: &'a [Token]) -> Parser {
    Parser {
      current_index: 0,
      tokens,
    }
  }

  fn expression(&mut self) -> Result<AstNode, String> {
    let mut results = vec![self.factor()];
    loop {
      self.advance();
      match self.current_token() {
        Token::Plus => {
          self.advance();
          results.push(self.factor())
        }
        Token::Minus => {
          self.advance();
          let minus1 = AstNode::new(AstHead::Number(-1.0), Vec::new());
          match self.factor() {
            Ok(neg) => results.push(Ok(AstNode::new(AstHead::Times, vec![minus1, neg]))),
            Err(error) => {
              results.push(Err(error));
            }
          }
        }
        Token::Eoi => {
          let (args, errors) = split_results(results);
          if errors.len() > 0 {
            return Err(unlines(errors));
          } else {
            return Ok(AstNode::new(AstHead::Plus, args));
          }
        }
        _ => {
          self.advance();
          results.push(Err("Expected to see a '+' or '-' after term".to_string()));
        }
      }
    }
  }

  fn factor(&mut self) -> Result<AstNode, String> {
    let mut results = vec![self.exponential()];
    loop {
      self.advance();
      match self.current_token() {
        Token::Plus | Token::Minus | Token::Eoi => {
          let (args, errors) = split_results(results);
          if errors.len() > 0 {
            return Err(unlines(errors));
          } else {
            return Ok(AstNode::new(AstHead::Times, args));
          }
        }
        Token::Star => {
          self.advance();
          results.push(self.exponential());
        }
        Token::Slash => {
          self.advance();
          let minus1 = AstNode::new(AstHead::Number(-1.0), Vec::new());
          match self.exponential() {
            Ok(denom) => {
              results.push(Ok(AstNode::new(AstHead::Power, vec![denom, minus1])));
            }
            Err(msg) => {
              results.push(Err(msg));
            }
          }
        }
        _ => {
          self.advance();
          results.push(Err(
            "Expected to see a '*' or '/' after factor.".to_string(),
          ));
        }
      }
    }
  }

  fn exponential(&mut self) -> Result<AstNode, String> {
    let mut results = vec![self.atom()];
    loop {
      self.advance();
      match self.current_token() {
        Token::Plus | Token::Star | Token::Slash | Token::Eoi => {
          let (args, errors) = split_results(results);
          if errors.len() > 0 {
            return Err(unlines(errors));
          } else {
            return Ok(AstNode::new(AstHead::Power, args));
          }
        }
        Token::Caret => {
          self.advance();
          results.push(self.atom());
        }
        _ => {
          self.advance();
          results.push(Err("Expected to see a '^' after a base".to_string()));
        }
      }
    }
  }

  fn atom(&mut self) -> Result<AstNode, String> {
    match self.current_token() {
      Token::LParen => {
        self.advance();
        let result = self.expression();
        match self.current_token() {
          Token::RParen => {
            self.advance();
            return result;
          }
          Token::Eoi => {
            return Err("Unbalanced parentheses.".to_string());
          }
          _ => unreachable!(),
        }
      }
      Token::Number(value) => Ok(AstNode::new(AstHead::Number(value), Vec::new())),
      _ => {
        return Err("Expected to see a number here".to_string());
      }
    }
  }

  fn advance(&mut self) {
    if self.current_index + 1 < self.tokens.len() {
      self.current_index += 1;
    }
  }

  fn current_token(&self) -> Token {
    if self.current_index < self.tokens.len() {
      self.tokens[self.current_index]
    } else {
      Token::Eoi
    }
  }
}
