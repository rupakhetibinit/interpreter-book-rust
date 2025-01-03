use std::io::Write;

use interpreter_book::{Lexer, Parser};

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

            let lexer = Lexer::new(input.as_str());
            let mut parser = Parser::new(lexer);
            if let Some(program) = parser.parse_program() {
                for stmt in program.statements.iter() {
                    println!("{}", stmt)
                }
            }
        }
    }
}
