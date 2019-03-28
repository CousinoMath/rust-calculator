//! The abstract syntax tree used for this calculator.
use std::collections::HashMap;
use std::f64;
use std::fmt;

/// An enumeration for the heads of the AST nodes.
#[derive(Clone, Debug, PartialEq)]
pub enum AstHead {
  Plus,
  Times,
  Power,
  Assign,
  Number(f64),
  Constant(String),
  Function(String),
  Identifier(String),
}

/// AST node structure: (AstHead AstNode*)
#[derive(Clone)]
pub struct AstNode {
  /// A tag to determine the type of AST node
  head: AstHead,
  /// A list of arguments/children of the node
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
      AstHead::Assign => write!(f, "(={})", tail_string),
      AstHead::Number(value) => write!(f, "{}", value),
      AstHead::Constant(name) => write!(f, "{}", name),
      AstHead::Function(name) => write!(f, "({}{})", name, tail_string),
      AstHead::Identifier(name) => write!(f, "{}", name),
    }
  }
}

impl AstNode {
  /// Creates a new AST
  pub fn new(head: AstHead, tail: Vec<AstNode>) -> AstNode {
    AstNode {
      head,
      tail: Box::new(tail),
    }
  }

  /// A helper function that creates an AST node for assignments.
  pub fn assign(name: &str, expr: AstNode) -> AstNode {
    AstNode::new(AstHead::Assign, vec![AstNode::identifier(name), expr])
  }

  /// Tests whether two ASTs are equal as trees.
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
      (AstHead::Identifier(id1), AstHead::Identifier(id2)) => id1 == id2,
      (AstHead::Function(name1), AstHead::Function(name2)) => {
        if name1 == name2 && self.tail.len() == other.tail.len() {
          let mut zipped = self.tail.iter().zip(other.tail.iter());
          zipped.all(|(a, b)| a.ast_equality(b))
        } else {
          false
        }
      }
      (_, _) => false,
    }
  }

  /// A helper function that creates an AST node for constants.
  /// This normalizes the string &ldquo;π&rdquo; as the ASCII &ldquo;pi&rdquo;
  pub fn constant(constant: &str) -> AstNode {
    let name = if constant == "π" { "pi" } else { constant };
    AstNode::new(AstHead::Constant(name.to_owned()), Vec::new())
  }

  /// Evaluates the AST using the state defined in `memory`.
  pub fn evaluate(&self, memory: &mut HashMap<String, f64>) -> f64 {
    let head = self.head.clone();
    let mut tail_iter = self.tail.iter();
    let mut identifier: Option<String> = None;
    if head == AstHead::Assign {
      let ident_node = tail_iter
        .next()
        .expect("Should have been an identifier as the first child of an assignment.");
      match ident_node.head.clone() {
        AstHead::Identifier(name) => {
          identifier = Some(name);
        }
        _ => unreachable!(),
      }
    }
    let evaled_tail = tail_iter
      .map(|arg| arg.evaluate(memory))
      .collect::<Vec<f64>>();
    match head {
      AstHead::Plus => evaled_tail.iter().sum(),
      AstHead::Times => evaled_tail.iter().product(),
      AstHead::Power => {
        if evaled_tail.len() == 0 {
          1.0_f64
        } else {
          let (first, rest) = evaled_tail.split_at(1);
          let first = first[0];
          rest.iter().rfold(first, |acc, &x| acc.powf(x))
        }
      }
      AstHead::Number(number) => number,
      AstHead::Constant(name) => match name.as_ref() {
        "pi" => f64::consts::PI,
        "e" => f64::consts::E,
        _ => f64::NAN,
      },
      AstHead::Function(name) => {
        let first = evaled_tail
          .get(0)
          .expect("Function should have been called with one argument");
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
      }
      AstHead::Identifier(name) => *memory.get(&name).unwrap_or(&f64::NAN),
      AstHead::Assign => {
        let ident_name =
          identifier.expect("Should have been an identifier as the first child to an assignment.");
        let ident_value = *evaled_tail
          .get(0)
          .expect("Should have been a value as the second child an assignment.");
        memory.insert(ident_name, ident_value);
        ident_value
      }
    }
  }

  /// A helper function that creates an AST node for functions.
  pub fn function(name: &str, argument: AstNode) -> AstNode {
    AstNode::new(AstHead::Function(name.to_string()), vec![argument])
  }

  /// A helper function that creates an AST node for identifiers.
  pub fn identifier(name: &str) -> AstNode {
    AstNode::new(AstHead::Identifier(name.to_owned()), Vec::new())
  }

  /// A helper function that creates an AST node for numbers
  pub fn number(value: f64) -> AstNode {
    AstNode::new(AstHead::Number(value), Vec::new())
  }

  /// A helper function that creates an AST node for addition
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

  /// A helper function that creates an AST node for exponentiation
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

  /// A helper function that creates an AST node for multiplication
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
}
