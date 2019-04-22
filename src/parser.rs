use crate::node::{BinExpNode, Node, TokenNode};
use crate::token::{Token, Tokens};
use std::collections::HashMap;

#[derive(Clone)]
enum Associativity {
    Right,
    Left,
}

fn get_precedence(op: &String) -> (u32, Associativity) {
    let mut map = HashMap::new();
    map.insert("=", (2, Associativity::Right));
    map.insert("+", (12, Associativity::Left));
    map.insert("*", (13, Associativity::Left));
    let op: &str = &op;
    let val = map[op].clone();
    val
}

pub fn parse_entry(mut tokens: Tokens) -> Node {
    let lhs = Node::Token(TokenNode::from_consume(&mut tokens));
    if let Ok(_) = tokens.expect("Op") {
        parse(lhs, &mut tokens)
    } else {
        lhs
    }
}

pub fn parse(mut lhs: Node, tokens: &mut Tokens) -> Node {
    while let Ok(Token::Op(op)) = tokens.expect("Op") {
        let (root_precedence, root_associativity) = get_precedence(&op);
        let op = match tokens.consume_op() {
            Ok(op) => op,
            Err(msg) => panic!(msg),
        };
        let mut rhs = Node::Token(TokenNode::from_consume(tokens));
        if let Ok(Token::Op(op2)) = tokens.expect("Op") {
            let (precedence, _associativity) = get_precedence(&op2);

            match root_associativity {
                Associativity::Right => {
                    if root_precedence <= precedence {
                        rhs = parse(rhs, tokens);
                    }
                }
                Associativity::Left => {
                    if root_precedence < precedence {
                        rhs = parse(rhs, tokens);
                    }
                }
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
    use crate::util::print_node;

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
    fn get_ide(ide: &str) -> Node {
        Node::Token(TokenNode {
            token: Token::Ide(String::from(ide)),
        })
    }
    fn get_bin_exp(op: &str, lhs: Node, rhs: Node) -> Node {
        Node::BinExp(BinExpNode {
            op: op.to_string(),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    #[allow(dead_code)]
    fn show(actual: Node, expected: Node) {
        println!("actual   ============");
        print_node(actual, 0, 0);
        println!("expected ============");
        print_node(expected, 0, 0);
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

    #[test]
    fn test_multi_terms_with_assign() {
        let actual = run(String::from("a = b = 1 + 2 * 3 + 4"));
        // expect: a = (b = ((1 + (2 * 3)) + 4)) but... -> TODO
        let lhs = get_ide("a");
        let rhs0 = get_bin_exp("*", get_num(2), get_num(3));
        let rhs1 = get_bin_exp("+", get_num(1), rhs0);
        let rhs2 = get_bin_exp("+", rhs1, get_num(4));
        let rhs = get_bin_exp("=", get_ide("b"), rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }
}
