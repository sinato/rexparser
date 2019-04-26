use crate::node::{BinExpNode, Node, Nodes, SuffixNode, TokenNode};
use crate::token::{Associativity, Token, Tokens};

pub fn prepare_nodes(mut tokens: Tokens) -> Nodes {
    let mut nodes: Vec<Node> = Vec::new();
    while let Some(token) = tokens.pop() {
        let node = match token.clone() {
            Token::SuffixOp(_, _) => {
                let node = match nodes.pop() {
                    Some(node) => node,
                    None => panic!("Expect an assinable."),
                };
                Node::Suffix(SuffixNode {
                    suffix: TokenNode { token },
                    node: Box::new(node),
                })
            }
            _ => Node::Token(TokenNode { token }),
        };
        nodes.push(node);
    }
    Nodes { nodes }
}

pub fn toplevel(tokens: Tokens) -> Node {
    let nodes = prepare_nodes(tokens);
    parse_entry(nodes)
}

pub fn parse_entry(mut nodes: Nodes) -> Node {
    let lhs = nodes.pop().unwrap();
    parse(lhs, &mut nodes, 0)
}

pub fn parse(mut lhs: Node, nodes: &mut Nodes, min_precedence: u32) -> Node {
    while let Some(node) = nodes.peek() {
        println!("======================");
        println!("{:?}", node);
        println!("======================");
        match node {
            Node::BinExp(_) | Node::Suffix(_) => panic!("Invalid expression."),
            Node::Token(node) => match node.token {
                Token::Op(op, property) => {
                    let (root_precedence, root_associativity) =
                        (property.clone().precedence, property.clone().associativity);
                    if root_precedence < min_precedence {
                        break;
                    }
                    nodes.pop(); // consume op
                    let op = TokenNode {
                        token: Token::Op(op, property),
                    };
                    // TODO: impl error handling
                    let mut rhs = nodes.pop().unwrap(); // read num/identifier
                    while let Some(Node::Token(TokenNode {
                        token: Token::Op(_, property2),
                    })) = nodes.peek()
                    {
                        let (precedence, _associativity) =
                            (property2.precedence, property2.associativity);
                        match root_associativity {
                            Associativity::Right => {
                                if root_precedence > precedence {
                                    break;
                                }
                            }
                            Associativity::Left => {
                                if root_precedence >= precedence {
                                    break;
                                }
                            }
                        }
                        rhs = parse(rhs, nodes, precedence)
                    }
                    lhs = Node::BinExp(BinExpNode {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    });
                }
                Token::Ide(_) | Token::Num(_) | Token::SuffixOp(_, _) => {
                    panic!("Invalid expression.")
                }
            },
        }
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
        toplevel(tokens)
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
    fn get_op(op: &str) -> TokenNode {
        let op = String::from(op);
        let property = lexer::get_property(&op);
        TokenNode {
            token: Token::Op(op, property),
        }
    }
    fn get_bin_exp(op: &str, lhs: Node, rhs: Node) -> Node {
        Node::BinExp(BinExpNode {
            op: get_op(op),
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
        // expect: (1 + (2 * 3)) + (4 * 5)
        let lhs1 = get_num(1);
        let lhs2 = get_bin_exp("*", get_num(2), get_num(3));
        let lhs = get_bin_exp("+", lhs1, lhs2);
        let rhs = get_bin_exp("*", get_num(4), get_num(5));
        let expected = get_bin_exp("+", lhs, rhs);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms_with_assign() {
        let actual = run(String::from("a = b = 1 + 2 * 3 + 4"));
        // expect: a = (b = ((1 + (2 * 3)) + 4))
        let lhs = get_ide("a");
        let rhs0 = get_bin_exp("*", get_num(2), get_num(3));
        let rhs1 = get_bin_exp("+", get_num(1), rhs0);
        let rhs2 = get_bin_exp("+", rhs1, get_num(4));
        let rhs = get_bin_exp("=", get_ide("b"), rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_suffix_increment() {
        let actual = run(String::from("a++"));
        // expect: a++
        let lhs = get_ide("a");
        let rhs0 = get_bin_exp("*", get_num(2), get_num(3));
        let rhs1 = get_bin_exp("+", get_num(1), rhs0);
        let rhs2 = get_bin_exp("+", rhs1, get_num(4));
        let rhs = get_bin_exp("=", get_ide("b"), rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }
}
