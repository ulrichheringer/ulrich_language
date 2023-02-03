use parser::Parser;

use std::io::Write;

use crate::interpreter::eval_program;

mod ast;
mod environment;
mod interpreter;
mod lexer;
mod parser;
mod values;

/*fn main() {
    let mut env = environment::Environment::new();
    env.push_var("abc".to_string(), "123".to_string());
    if let Some(result) = env.lookup_var("abc".to_string()) {
        println!("{}", result);
    };
    let mut parserr = parser::Parser::new();
    let program = parserr.produce_ast("let abc = \"simple string\" 123 abc".to_string());
    for i in &program.body {
        println!("{:#?}", i);
    }
}*/

fn main() {
    let mut parser = Parser::new();
    println!("REPL v0.1");
    std::process::Command::new("clear").status().unwrap();
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
        if let Some(values::RuntimeVal::NumberVal { kind: _, value }) = result {
            println!("{}", value);
        } else if let Some(values::RuntimeVal::IdentVal { kind: _, value }) = result {
            println!("{}", value);
        } else if let Some(values::RuntimeVal::TextVal { value }) = result {
            println!("\"{}\"", value);
        } else {
            println!("nil");
        }
    }
}
