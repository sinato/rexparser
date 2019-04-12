mod lexer;
use lexer::{Token, Tokens};

#[derive(Debug, PartialEq)]
enum Node {
    Add(AddNode),
    Mul(MulNode),
}

#[derive(Debug, PartialEq)]
struct AddNode {
    lhs: Token,
    rhs: Token,
}

#[derive(Debug, PartialEq)]
struct MulNode {
    lhs: Token,
    rhs: Token,
}

fn parse(mut tokens: Tokens) -> Node {
    let lhs = match tokens.consume("Num") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    let op = match tokens.consume("Op") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    let rhs = match tokens.consume("Num") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    match op {
        Token::Op(op) => match op.as_ref() {
            "+" => Node::Add(AddNode { lhs, rhs }),
            "*" => Node::Mul(MulNode { lhs, rhs }),
            _ => panic!(),
        },
        _ => panic!(),
    }
}

/// expression := num op num
fn main() {
    let input = String::from("1 * 2");
    let lexer = lexer::Lexer::new();
    let tokens = lexer.lex(input);
    println!("tokens: {:?}", tokens);
    let node = parse(tokens);
    println!("node: {:?}", node);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: String) -> Node {
        let lexer = lexer::Lexer::new();
        let tokens = lexer.lex(input);
        parse(tokens)
    }

    #[test]
    fn test_add() {
        let actual = run(String::from("1 + 2"));
        let expected = Node::Add(AddNode {
            lhs: Token::Num(1),
            rhs: Token::Num(2),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        let actual = run(String::from("1 * 2"));
        let expected = Node::Mul(MulNode {
            lhs: Token::Num(1),
            rhs: Token::Num(2),
        });
        assert_eq!(actual, expected);
    }
}
