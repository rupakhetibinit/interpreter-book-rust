use core::ast::Parser;
#[allow(unused, dead_code)]
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

            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if parser.errors().is_empty() {
                println!("{:?}", program)
            } else {
                for error in parser.errors() {
                    println!("{}", error);
                }
            }
        }
    }
}
