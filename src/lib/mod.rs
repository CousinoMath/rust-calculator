pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

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

pub fn unlines(lines: Vec<String>) -> String {
  lines.iter().fold("".to_string(), |acc, x| acc + "\n" + x)
}
