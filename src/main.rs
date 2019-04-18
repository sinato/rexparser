mod lexer;
mod token;

use token::{Token, Tokens};

#[derive(Debug, PartialEq)]
enum Node {
    Add(AddNode),
    Mul(MulNode),
    Token(TokenNode),
}

#[derive(Debug, PartialEq)]
struct AddNode {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

#[derive(Debug, PartialEq)]
struct MulNode {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

#[derive(Debug, PartialEq)]
struct TokenNode {
    token: Token,
}

fn parse(mut tokens: Tokens) -> Node {
    let mut lhs = match tokens.consume("Num") {
        Ok(token) => Node::Token(TokenNode { token }),
        Err(msg) => panic!(msg),
    };
    while let Ok(Token::Op(_)) = tokens.expect("Op") {
        let op2 = match tokens.consume("Op") {
            Ok(token) => token,
            Err(msg) => panic!(msg),
        };
        let term3 = match tokens.consume("Num") {
            Ok(token) => Box::new(Node::Token(TokenNode { token })),
            Err(msg) => panic!(msg),
        };
        lhs = match op2 {
            Token::Op(op) => match op.as_ref() {
                "+" => Node::Add(AddNode {
                    lhs: Box::new(lhs),
                    rhs: term3,
                }),
                "*" => Node::Mul(MulNode {
                    lhs: Box::new(lhs),
                    rhs: term3,
                }),
                _ => panic!(),
            },
            _ => panic!(),
        };
    }
    lhs
}

/// expression := num op num (op num)?
fn main() {
    let input = String::from("1 * 2 + 3 * 4");
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

    fn get_num(num: i32) -> Node {
        Node::Token(TokenNode {
            token: Token::Num(num),
        })
    }
    fn get_add_exp(lhs: Node, rhs: Node) -> Node {
        Node::Add(AddNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    fn get_mul_exp(lhs: Node, rhs: Node) -> Node {
        Node::Mul(MulNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    #[test]
    fn test_add() {
        let actual = run(String::from("1 + 2"));
        let lhs = get_num(1);
        let rhs = get_num(2);
        let expected = get_add_exp(lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        let actual = run(String::from("1 * 2"));
        let lhs = get_num(1);
        let rhs = get_num(2);
        let expected = get_mul_exp(lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_three_terms() {
        let actual = run(String::from("1 * 2 + 3"));
        let lhs = get_num(1);
        let rhs = get_num(2);

        let lhs = get_mul_exp(lhs, rhs);
        let rhs = get_num(3);

        let expected = get_add_exp(lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms() {
        let actual = run(String::from("1 * 2 + 3 * 4"));
        let lhs = get_num(1);
        let rhs = get_num(2);

        let lhs = get_mul_exp(lhs, rhs);
        let rhs = get_num(3);

        let lhs = get_add_exp(lhs, rhs);
        let rhs = get_num(4);

        let expected = get_mul_exp(lhs, rhs);
        assert_eq!(actual, expected);
    }
}
