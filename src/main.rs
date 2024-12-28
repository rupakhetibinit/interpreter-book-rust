use core::{lexer::Lexer, token::TokenType};
use std::io::{Write, stdin};

mod core;

fn main() {
    let mut args = std::env::args();
    args.next();

    println!("Hello! This is the Monkey programming language written in Rust!");
    println!("Feel free to type in commands.");
    if args.next().unwrap() == "repl" {
        loop {
            let mut input = String::new();
            print!("> ");
            let _ = std::io::stdout().flush();
            stdin().read_line(&mut input).unwrap();

            if input.trim_end().is_empty() {
                println!("Exiting Repl");
                std::process::exit(0)
            }

            let mut lexer = Lexer::new(input);

            'a: loop {
                let token = lexer.next_token();
                if token.token_type == TokenType::Eof {
                    break 'a;
                }
                println!("{:?}", token)
            }
        }
    }
}
