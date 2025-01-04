use std::io::Write;

use interpreter_book::{Lexer, Parser, eval_program};

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
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim_end().is_empty() {
                println!("Exiting Repl");
                std::process::exit(0)
            }

            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            if let Some(mut program) = parser.parse_program() {
                if parser.errors.is_empty() {
                    println!("{}", eval_program(&mut program));
                } else {
                    for error in parser.errors.iter() {
                        println!("{}", error)
                    }
                }
            }
        }
    }
}
