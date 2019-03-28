//! The parser for the calculator

use crate::lib::ast::AstNode;
use crate::lib::token::Token;
use crate::lib::{split_results, unlines};

/// The parser state
pub struct Parser<'a> {
  /// Current index in the slice of tokens
  current_index: usize,
  /// Slice of tokens
  tokens: &'a [Token],
}

impl<'a> Parser<'a> {
  /// Advances the parser one token
  fn advance(&mut self) {
    if self.current_index + 1 < self.tokens.len() {
      self.current_index += 1;
    }
  }

  /// Parses the rule for assignmnent
  /// assignment ::= identifier '=' expression
  ///            | expression
  fn assignment(&mut self) -> Result<AstNode, String> {
    let curr_token = self.current_token();
    if let Token::Identifier(id) = curr_token {
      if self.peek(1) == Token::Equals {
        self.advance();
        self.advance();
        let result = self.expression();
        return result.map(|expr| AstNode::assign(&id, expr));
      }
    }
    self.expression()
  }

  /// Parses atoms
  /// atom ::= '(' expression ')'
  ///      | Function atom
  ///      | Number
  ///      | Identifier
  ///      | Constant
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
      Token::Number(value) => {
        self.advance();
        Ok(AstNode::number(value))
      }
      Token::Constant(constant) => {
        self.advance();
        Ok(AstNode::constant(&constant))
      }
      Token::Identifier(identifier) => {
        self.advance();
        Ok(AstNode::identifier(&identifier))
      }
      Token::Function(function) => {
        self.advance();
        let result = self.atom();
        match result {
          Ok(ast) => Ok(AstNode::function(&function, ast)),
          Err(msg) => Err(msg),
        }
      }
      _ => {
        return Err(format!(
          "Expected to see a number here {}",
          self.current_token()
        ));
      }
    }
  }

  /// Returns the current token under consideration
  fn current_token(&self) -> Token {
    if self.current_index < self.tokens.len() {
      self.tokens[self.current_index].clone()
    } else {
      Token::Eoi
    }
  }

  /// Parses the rule for exponentials
  /// exponential ::= atom ('^' atom)*
  ///             | '-' exponential
  fn exponential(&mut self) -> Result<AstNode, String> {
    let mut results: Vec<Result<AstNode, String>> = Vec::new();
    match self.current_token() {
      Token::Minus => {
        self.advance();
        let minus_1 = AstNode::number(-1.0);
        results.push(self.exponential().map(|node| AstNode::times(vec![minus_1, node])));
      }
      _ => results.push(self.atom()),
    }
    loop {
      match self.current_token() {
        Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Eoi | Token::RParen => {
          let (args, errors) = split_results(results);
          if errors.len() > 0 {
            return Err(unlines(errors).trim().to_string());
          } else {
            return Ok(AstNode::power(args));
          }
        }
        Token::Caret => {
          self.advance();
          results.push(self.exponential());
        }
        _ => {
          return Err(format!(
            "Expected to see a '^' after base {}",
            self.current_token()
          ));
        }
      }
    }
  }

  /// Parses the rule for expressions
  /// expression ::= term (('+' | '-') term)*
  fn expression(&mut self) -> Result<AstNode, String> {
    let mut results = vec![self.factor()];
    loop {
      match self.current_token() {
        Token::Plus => {
          self.advance();
          results.push(self.factor())
        }
        Token::Minus => {
          self.advance();
          let minus1 = AstNode::number(-1.0);
          match self.factor() {
            Ok(neg) => results.push(Ok(AstNode::times(vec![minus1, neg]))),
            Err(error) => {
              results.push(Err(error));
            }
          }
        }
        Token::Eoi | Token::RParen => {
          let (args, errors) = split_results(results);
          if errors.len() > 0 {
            return Err(unlines(errors).trim().to_string());
          } else {
            return Ok(AstNode::plus(args));
          }
        }
        _ => {
          return Err(format!(
            "Expected to see a '+' or '-' after term {}",
            self.current_token()
          ));
        }
      }
    }
  }

  /// Parses the rule for factors
  /// factor ::= exponential (('*' | '/') exponential)*
  fn factor(&mut self) -> Result<AstNode, String> {
    let mut results = vec![self.exponential()];
    loop {
      match self.current_token() {
        Token::Plus | Token::Minus | Token::Eoi | Token::RParen => {
          let (args, errors) = split_results(results);
          if errors.len() > 0 {
            return Err(unlines(errors).trim().to_string());
          } else {
            return Ok(AstNode::times(args));
          }
        }
        Token::Star => {
          self.advance();
          results.push(self.exponential());
        }
        Token::Slash => {
          self.advance();
          let minus1 = AstNode::number(-1.0);
          match self.exponential() {
            Ok(denom) => {
              results.push(Ok(AstNode::power(vec![denom, minus1])));
            }
            Err(msg) => {
              results.push(Err(msg));
            }
          }
        }
        _ => {
          return Err(format!(
            "Expected to see a '*' or '/' after factor {}",
            self.current_token()
          ));
        }
      }
    }
  }

  /// Initializes parser state on a slice of tokens.
  fn new(tokens: &'a [Token]) -> Parser {
    Parser {
      current_index: 0,
      tokens,
    }
  }

  /// Parses a slice of tokens into an abstract syntax tree.
  pub fn parse(tokens: &'a [Token]) -> Result<AstNode, String> {
    let mut parser = Parser::new(tokens);
    match parser.assignment() {
      Ok(ast) => Ok(ast),
      Err(msg) => Err(msg),
    }
  }

  /// Peeks at the `step`th token ahead. Used in the assignment rule.
  fn peek(&self, step: usize) -> Token {
    if self.current_index + step < self.tokens.len() {
      self.tokens[self.current_index + step].clone()
    } else {
      Token::Eoi
    }
  }
}

#[cfg(test)]
mod test {
  use crate::lib::ast::AstNode;
  use crate::lib::parser::Parser;
  use crate::lib::token::Token;
  #[test]
  fn parse_number() {
    let value = 1.0;
    let tokens = [Token::Number(value)];
    let ast_result = Parser::parse(&tokens[..]);
    assert!(ast_result.is_ok());
    assert!(ast_result.unwrap().ast_equality(&AstNode::number(value)));
  }

  #[test]
  fn parse_op() {
    let a = 1.0;
    let b = 2.0;
    let op = Token::Plus;
    let tokens = [Token::Number(a), op, Token::Number(b)];
    let ast_result = Parser::parse(&tokens[..]);
    assert!(ast_result.is_ok());
    assert!(ast_result.unwrap().ast_equality(&AstNode::plus(
      [AstNode::number(a), AstNode::number(b)].to_vec()
    )));
  }
}
