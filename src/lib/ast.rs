use std::f64;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum AstHead {
  Plus,
  Times,
  Power,
  Number(f64),
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
    match self.head {
      AstHead::Plus => write!(f, "(+{})", tail_string),
      AstHead::Times => write!(f, "(*{})", tail_string),
      AstHead::Power => write!(f, "(^{})", tail_string),
      AstHead::Number(value) => write!(f, "{}", value),
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
    match self.head {
      AstHead::Plus => evaled_tail.iter().sum(),
      AstHead::Times => evaled_tail.iter().product(),
      AstHead::Power => {
        if evaled_tail.len() == 0 {
          1.0
        } else {
          let (first, rest) = evaled_tail.split_at(1);
          let first = first[0];
          rest.iter().rfold(first, |acc, x| f64::powf(*x, acc))
        }
      }
      AstHead::Number(number) => number,
    }
  }

  pub fn ast_equality(&self, other: &Self) -> bool {
    match (self.head, other.head) {
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
        AstHead::Number(_) => self,
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
      0 => AstNode::new(AstHead::Number(0.0), Vec::new()),
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
      0 => AstNode::new(AstHead::Number(1.0), Vec::new()),
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
      0 => AstNode::new(AstHead::Number(1.0), Vec::new()),
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
}
