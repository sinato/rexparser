mod lexer;
use lexer::{Token, Tokens};

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let input = String::from("1 + 2");
        let lexer = lexer::Lexer::new();
        let tokens = lexer.lex(input);
        let node = parse(tokens);

        let expected = AddNode {
            lhs: Token::Num(1),
            rhs: Token::Num(2),
        };

        assert_eq!(node, expected);
    }
}
