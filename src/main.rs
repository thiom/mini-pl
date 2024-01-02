mod interpreter;
mod nodes;
mod parser;
mod scanner;
mod tokens;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap();
    let source = command_line.next().unwrap();
    let mut file = std::fs::File::open(source).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    if !input.is_empty() {
        let lexer = Scanner::new(input);
        let parser = Parser::new(lexer);
        let mut interpreter = Interpreter::new(parser);
        let result = interpreter.interpret();
        println!("{}", result);
        Ok(())
    } else {
        println!("No input received");
        Ok(())
    }
}
