//! The library consists of modules for the lexical tokens, lexical analyzer,
//! parser, and abstract syntax tree used in this calculator.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

/// Takes a vector of results and splits into two
/// vectors, the first for successes (`Ok`s) and the second for errors.
pub fn split_results<A, B>(results: Vec<Result<A, B>>) -> (Vec<A>, Vec<B>) {
  let mut oks: Vec<A> = Vec::new();
  let mut errors: Vec<B> = Vec::new();
  for result in results {
    match result {
      Ok(result) => {
        oks.push(result);
      }
      Err(error) => {
        errors.push(error);
      }
    }
  }
  (oks, errors)
}

/// Concatenates a vector of strings with newlines `\n`.
pub fn unlines(lines: Vec<String>) -> String {
  lines.iter().fold("".to_string(), |acc, x| acc + "\n" + x)
}
