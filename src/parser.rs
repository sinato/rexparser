use crate::token::{Token, Tokens};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BinExp(BinExpNode),
    Token(TokenNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinExpNode {
    pub op: String,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TokenNode {
    pub token: Token,
}

fn get_precedence(op: &String) -> u32 {
    let mut map = HashMap::new();
    map.insert("+", 12);
    map.insert("*", 13);
    let op: &str = &op;
    map[op]
}

pub fn parse_entry(mut tokens: Tokens) -> Node {
    let lhs = match tokens.consume("Num") {
        Ok(token) => Node::Token(TokenNode { token }),
        Err(msg) => panic!(msg),
    };
    if let Ok(Token::Op(op)) = tokens.expect("Op") {
        println!("entry op: {:?}", op);
        parse(lhs, &mut tokens)
    } else {
        lhs
    }
}

pub fn parse(mut lhs: Node, tokens: &mut Tokens) -> Node {
    while let Ok(Token::Op(op)) = tokens.expect("Op") {
        let root_precedence = get_precedence(&op);
        let op = match tokens.consume("Op") {
            Ok(token) => match token {
                Token::Op(op) => op,
                _ => panic!("Unexptected pattern."),
            },
            Err(msg) => panic!(msg),
        };

        let mut rhs = match tokens.consume("Num") {
            Ok(token) => Node::Token(TokenNode { token }),
            Err(msg) => panic!(msg),
        };
        if let Ok(Token::Op(op2)) = tokens.expect("Op") {
            let precedence = get_precedence(&op2);
            if root_precedence < precedence {
                rhs = parse(rhs, tokens);
            }
        }
        lhs = Node::BinExp(BinExpNode {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });
    }
    lhs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    fn run(input: String) -> Node {
        let lexer = lexer::Lexer::new();
        let tokens = lexer.lex(input);
        parse_entry(tokens)
    }

    fn get_num(num: i32) -> Node {
        Node::Token(TokenNode {
            token: Token::Num(num),
        })
    }
    fn get_bin_exp(op: &str, lhs: Node, rhs: Node) -> Node {
        Node::BinExp(BinExpNode {
            op: op.to_string(),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    #[test]
    fn test_add() {
        let actual = run(String::from("1 + 2"));
        let lhs = get_num(1);
        let rhs = get_num(2);
        let expected = get_bin_exp("+", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        let actual = run(String::from("1 * 2"));
        let lhs = get_num(1);
        let rhs = get_num(2);
        let expected = get_bin_exp("*", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_three_terms() {
        let actual = run(String::from("1 * 2 + 3"));
        let lhs = get_num(1);
        let rhs = get_num(2);

        let lhs = get_bin_exp("*", lhs, rhs);
        let rhs = get_num(3);

        let expected = get_bin_exp("+", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms() {
        let actual = run(String::from("1 * 2 + 3 * 4"));
        // expect: (1 * 2) + (3 * 4)
        let lhs1 = get_num(1);
        let rhs1 = get_num(2);
        let lhs = get_bin_exp("*", lhs1, rhs1);

        let lhs2 = get_num(3);
        let rhs2 = get_num(4);
        let rhs = get_bin_exp("*", lhs2, rhs2);

        let expected = get_bin_exp("+", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms2() {
        let actual = run(String::from("1 + 2 * 3 + 4 * 5"));
        // expect: 1 + ((2 * 3) + (4 * 5))
        let lhs = get_num(1);

        let rhs1 = get_bin_exp("*", get_num(2), get_num(3));
        let rhs2 = get_bin_exp("*", get_num(4), get_num(5));
        let rhs = get_bin_exp("+", rhs1, rhs2);
        let expected = get_bin_exp("+", lhs, rhs);

        assert_eq!(actual, expected);
    }
}
