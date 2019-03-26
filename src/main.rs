use std::io::{self, Write};

pub mod lib;

use crate::lib::lexer::Lexer;

enum State {
    Continue,
    Exit,
}

fn main() -> io::Result<()> {
    loop {
        match read_line() {
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

fn read_line() -> io::Result<State> {
    print!("> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.len() == 0 {
        Ok(State::Exit)
    } else {
        match Lexer::lex(input.trim()) {
            Ok(tokens) => {
                for token in tokens {
                    print!("{}", token);
                }
                println!("");
            }
            Err(message) => println!("{}", message),
        }
        Ok(State::Continue)
    }
}
