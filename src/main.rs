mod lexer;
use lexer::{Token, Tokens};

#[derive(Debug)]
struct AddNode {
    lhs: Token,
    rhs: Token,
}

fn parse(mut tokens: Tokens) -> AddNode {
    let lhs = match tokens.consume("Num") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    let _op = tokens.consume("Op");
    let rhs = match tokens.consume("Num") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    AddNode { lhs, rhs }
}

fn main() {
    let input = String::from("1 + 2");
    let lexer = lexer::Lexer::new();
    let tokens = lexer.lex(input);
    println!("tokens: {:?}", tokens);
    let node = parse(tokens);
    println!("node: {:?}", node);
}
