use crate::lib::ast::AstNode;
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

  fn exponential(&mut self) -> Result<AstNode, String> {
    let mut results = vec![self.atom()];
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
          results.push(self.atom());
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
        Ok(AstNode::constant(& constant))
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

  fn advance(&mut self) {
    if self.current_index + 1 < self.tokens.len() {
      self.current_index += 1;
    }
  }

  fn current_token(&self) -> Token {
    if self.current_index < self.tokens.len() {
      self.tokens[self.current_index].clone()
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
