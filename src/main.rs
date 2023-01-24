use parser::Parser;

use std::io::Write;

use crate::interpreter::eval_program;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod values;

fn main() {
    let mut parser = Parser::new();
    println!("REPL v0.1");
    loop {
        let mut input = String::new();
        print!("> ");

        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        std::io::stdout().flush().unwrap();

        if input == "exit\r\n" {
            println!("Exited sucessfully");
            break;
        }
        let program = parser.produce_ast(input);
        let result = eval_program(program);
        println!("{:#?}", result);
    }
}
