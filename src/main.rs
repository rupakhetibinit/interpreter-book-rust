use std::io::Write;

use bumpalo::Bump;
use interpreter_book::{Lexer, Parser, StringArena};

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
            let allocator = Bump::new();
            let arena = StringArena::new(&allocator);
            let program = parser.parse_program(arena);
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
