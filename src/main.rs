mod lexer;
mod token;

use token::{Token, Tokens};

#[derive(Debug, PartialEq)]
enum Node {
    Add(AddNode),
    Mul(MulNode),
    Token(Token),
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

fn parse(mut tokens: Tokens) -> Node {
    let term1 = match tokens.consume("Num") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    let op1 = match tokens.consume("Op") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    let term2 = match tokens.consume("Num") {
        Ok(token) => token,
        Err(msg) => panic!(msg),
    };
    let lhs = match op1 {
        Token::Op(op) => match op.as_ref() {
            "+" => Node::Add(AddNode {
                lhs: Box::new(Node::Token(term1)),
                rhs: Box::new(Node::Token(term2)),
            }),
            "*" => Node::Mul(MulNode {
                lhs: Box::new(Node::Token(term1)),
                rhs: Box::new(Node::Token(term2)),
            }),
            _ => panic!(),
        },
        _ => panic!(),
    };
    if let Ok(Token::Op(_)) = tokens.expect("Op") {
        let op2 = match tokens.consume("Op") {
            Ok(token) => token,
            Err(msg) => panic!(msg),
        };
        let term3 = match tokens.consume("Num") {
            Ok(token) => token,
            Err(msg) => panic!(msg),
        };
        match op2 {
            Token::Op(op) => match op.as_ref() {
                "+" => Node::Add(AddNode {
                    lhs: Box::new(lhs),
                    rhs: Box::new(Node::Token(term3)),
                }),
                "*" => Node::Mul(MulNode {
                    lhs: Box::new(lhs),
                    rhs: Box::new(Node::Token(term3)),
                }),
                _ => panic!(),
            },
            _ => panic!(),
        }
    } else {
        lhs
    }
}

/// expression := num op num (op num)?
fn main() {
    let input = String::from("1 * 2 + 3");
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
        Node::Token(Token::Num(num))
    }
    fn get_add_exp(num1: i32, num2: i32) -> Node {
        Node::Add(AddNode {
            lhs: Box::new(get_num(num1)),
            rhs: Box::new(get_num(num2)),
        })
    }
    fn get_mul_exp(num1: i32, num2: i32) -> Node {
        Node::Mul(MulNode {
            lhs: Box::new(get_num(num1)),
            rhs: Box::new(get_num(num2)),
        })
    }

    #[test]
    fn test_add() {
        let actual = run(String::from("1 + 2"));
        let expected = get_add_exp(1, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        let actual = run(String::from("1 * 2"));
        let expected = get_mul_exp(1, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_three_terms() {
        let actual = run(String::from("1 * 2 + 3"));
        let lhs = get_mul_exp(1, 2);
        let expected = Node::Add(AddNode {
            lhs: Box::new(lhs),
            rhs: Box::new(get_num(3)),
        });
        assert_eq!(actual, expected);
    }
}
