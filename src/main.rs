use parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let mut result = Parser::new();
    println!("{:#?}", result.produce_ast("(1+2)+3".to_string()));
}
