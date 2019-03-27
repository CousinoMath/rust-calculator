use std::f64;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum AstHead {
  Plus,
  Times,
  Power,
  Number(f64),
  Constant(String),
  Function(String),
  Identifier(String),
}

#[derive(Clone)]
pub struct AstNode {
  head: AstHead,
  tail: Box<Vec<AstNode>>,
}

impl fmt::Display for AstNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let tail_string = self
      .tail
      .iter()
      .fold("".to_string(), |acc, x| format!("{} {}", acc, x));
    match self.head.clone() {
      AstHead::Plus => write!(f, "(+{})", tail_string),
      AstHead::Times => write!(f, "(*{})", tail_string),
      AstHead::Power => write!(f, "(^{})", tail_string),
      AstHead::Number(value) => write!(f, "{}", value),
      AstHead::Constant(name) => write!(f, "{}", name),
      AstHead::Function(name) => write!(f, "({}{})", name, tail_string),
      AstHead::Identifier(name) => write!(f, "{}", name),
    }
  }
}

impl AstNode {
  pub fn new(head: AstHead, tail: Vec<AstNode>) -> AstNode {
    AstNode {
      head,
      tail: Box::new(tail),
    }
  }

  pub fn evaluate(&self) -> f64 {
    let evaled_tail: Vec<f64> = self.tail.iter().map(|arg| arg.evaluate()).collect();
    match self.head.clone() {
      AstHead::Plus => evaled_tail.iter().sum(),
      AstHead::Times => evaled_tail.iter().product(),
      AstHead::Power => {
        if evaled_tail.len() == 0 {
          1.0_f64
        } else {
          let (first, rest) = evaled_tail.split_at(1);
          let first = first[0];
          rest.iter().rfold(first, |acc, x| x.powf(acc))
        }
      },
      AstHead::Number(number) => number,
      AstHead::Constant(name) => {
        match name.as_ref() {
          "pi" => f64::consts::PI,
          "e" => f64::consts::E,
          _ => f64::NAN,
        }
      },
      AstHead::Function(name) => {
        let first = evaled_tail.get(0).expect("Function should have been called with one argument");
        match name.as_ref() {
          "abs" => first.abs(),
          "acos" => first.acos(),
          "acosh" => first.acosh(),
          "asin" => first.asin(),
          "asinh" => first.asinh(),
          "atan" => first.atan(),
          "atanh" => first.atanh(),
          "cos" => first.cos(),
          "cosh" => first.cosh(),
          "exp" => first.exp(),
          "log" => first.ln(),
          "sin" => first.sin(),
          "sinh" => first.sinh(),
          "sqrt" => first.sqrt(),
          "tan" => first.tan(),
          "tanh" => first.tanh(),
          _ => f64::NAN,
        }
      },
      AstHead::Identifier(_) => f64::NAN,
    }
  }

  pub fn ast_equality(&self, other: &Self) -> bool {
    match (self.head.clone(), other.head.clone()) {
      (AstHead::Plus, AstHead::Plus)
      | (AstHead::Times, AstHead::Times)
      | (AstHead::Power, AstHead::Power) => {
        if self.tail.len() == other.tail.len() {
          let mut zipped = self.tail.iter().zip(other.tail.iter());
          zipped.all(|(a, b)| a.ast_equality(b))
        } else {
          false
        }
      }
      (AstHead::Number(value1), AstHead::Number(value2)) => value1 == value2,
      (_, _) => false,
    }
  }

  pub fn prune(&self) -> &AstNode {
    let tail_len = self.tail.len();
    if tail_len == 1 {
      match self.head {
        AstHead::Plus | AstHead::Times | AstHead::Power => self
          .tail
          .get(0)
          .expect("Should be able to get 0th element of non-empty vector."),
        AstHead::Number(_) | AstHead::Constant(_) | AstHead::Function(_) | AstHead::Identifier(_) => self,
      }
    } else {
      self
    }
  }

  pub fn is_atom(&self) -> bool {
    self.tail.len() == 0
  }

  pub fn plus(arguments: Vec<AstNode>) -> AstNode {
    let len = arguments.len();
    match len {
      0 => AstNode::number(0.0),
      1 => arguments
        .get(0)
        .expect("Should be able to get 0th element of a non-empty vector.")
        .clone(),
      _ => AstNode::new(AstHead::Plus, arguments),
    }
  }

  pub fn times(arguments: Vec<AstNode>) -> AstNode {
    let len = arguments.len();
    match len {
      0 => AstNode::number(1.0),
      1 => arguments
        .get(0)
        .expect("Should be able to get 0th element of a non-empty vector.")
        .clone(),
      _ => AstNode::new(AstHead::Times, arguments),
    }
  }

  pub fn power(arguments: Vec<AstNode>) -> AstNode {
    let len = arguments.len();
    match len {
      0 => AstNode::number(1.0),
      1 => arguments
        .get(0)
        .expect("Should be able to get 0th element of a non-empty vector.")
        .clone(),
      _ => {
        let last_rest = arguments
          .split_last()
          .expect("Should be able to split the last element off a non-empty vector.");
        let (last, rest) = (last_rest.0.clone(), last_rest.1.clone());
        rest.iter().rfold(last, |acc, x| {
          AstNode::new(AstHead::Power, vec![x.clone(), acc])
        })
      }
    }
  }

  pub fn number(value: f64) -> AstNode {
    AstNode::new(AstHead::Number(value), Vec::new())
  }

  pub fn constant(constant: & str) -> AstNode {
    let name = if constant == "π" { "pi" } else { constant };
    AstNode::new(AstHead::Constant(name.to_owned()), Vec::new())
  }

  pub fn function(name: &str, argument: AstNode) -> AstNode {
    AstNode::new(AstHead::Function(name.to_string()), vec![argument])
  }

  pub fn identifier(name: &str) -> AstNode {
    AstNode::new(AstHead::Identifier(name.to_owned()), Vec::new())
  }
}
