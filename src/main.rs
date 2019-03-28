use std::collections::HashMap;
use std::io::{self, Write};

pub mod lib;

use crate::lib::lexer::Lexer;
use crate::lib::parser::Parser;

/// A simple enumeration to determine if the program should continue or halt.
/// The program halts on empty input.
enum State {
    Continue,
    Exit,
}

fn main() -> io::Result<()> {
    let mut memory: HashMap<String, f64> = HashMap::new();
    loop {
        match read_line(&mut memory) {
            Ok(State::Continue) => continue,
            Ok(State::Exit) => break,
            Err(err) => {
                eprintln!("{:#?}", err);
                return Err(err);
            }
        }
    }
    Ok(())
}

/// Reads the current line of input and evaluates it. The state that it returns
/// indicates whether or not the main program should continue.
fn read_line(memory: &mut HashMap<String, f64>) -> io::Result<State> {
    print!("> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    if input.len() == 0 {
        Ok(State::Exit)
    } else {
        match Lexer::lex(&input) {
            Ok(tokens) => {
                for token in tokens.clone() {
                    print!("{}", token);
                }
                println!("");
                match Parser::parse(tokens.as_slice()) {
                    Ok(ast) => println!("{} = {}", ast, ast.evaluate(memory)),
                    Err(message) => eprintln!("{}", message),
                }
            }
            Err(message) => eprintln!("{}", message),
        }
        Ok(State::Continue)
    }
}
