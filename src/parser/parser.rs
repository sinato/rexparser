use crate::lexer::token::{Associativity, Token, Tokens};
use crate::parser::node::{
    ArrayIndexNode, BinExpNode, FunctionCallNode, Node, SuffixNode, TernaryExpNode, TokenNode,
};

pub fn toplevel(mut tokens: Tokens) -> Node {
    expression(&mut tokens)
}

pub fn expression(tokens: &mut Tokens) -> Node {
    let lhs = primary(tokens);
    binary_expression(lhs, tokens, 0)
}

fn primary(tokens: &mut Tokens) -> Node {
    let lhs = match tokens.pop() {
        Some(token) => match token {
            Token::Ide(_) | Token::Num(_) => Node::Token(TokenNode { token }),
            _ => panic!(format!("Expect a primary token, but this is {:?}", token)),
        },
        None => panic!(),
    };
    let lhs = match tokens.peek() {
        Some(token) => match token {
            Token::SuffixOp(_, _) => suffix(lhs, tokens),
            Token::Question => {
                // ternary expression
                let condition = lhs.clone();
                let _question = tokens.pop();
                let ternary_lhs = expression(tokens);
                let _colon = tokens.pop();
                let ternary_rhs = expression(tokens);
                Node::TernaryExp(TernaryExpNode {
                    condition: Box::new(condition),
                    lhs: Box::new(ternary_lhs),
                    rhs: Box::new(ternary_rhs),
                })
            }
            Token::Op(_, _) => lhs,
            _ => lhs,
        },
        None => lhs,
    };
    lhs
}

fn suffix(lhs: Node, tokens: &mut Tokens) -> Node {
    match tokens.pop().unwrap() {
        Token::SuffixOp(suffix, property) => match suffix.as_ref() {
            "++" => Node::Suffix(SuffixNode {
                suffix: TokenNode {
                    token: Token::SuffixOp(suffix, property),
                },
                node: Box::new(lhs),
            }),
            "[" => {
                let index = expression(tokens);
                Node::ArrayIndex(ArrayIndexNode {
                    array: Box::new(lhs),
                    index: Box::new(index),
                })
            }
            "(" => {
                if let Node::Token(token_node) = lhs {
                    let parameters = match tokens.peek() {
                        Some(token) => match token {
                            Token::ParenE => Box::new(Node::Empty),
                            _ => Box::new(expression(tokens)),
                        },
                        None => panic!(),
                    };
                    tokens.pop(); // consume ParanE TODO: impl error handling
                    Node::FunctionCall(FunctionCallNode {
                        identifier: token_node,
                        parameters,
                    })
                } else {
                    panic!("Expect a token node as lhs.")
                }
            }
            _ => panic!(),
        },
        _ => panic!("Expect a suffix operator."),
    }
}

pub fn binary_expression(mut lhs: Node, tokens: &mut Tokens, min_precedence: u32) -> Node {
    while let Some(token) = tokens.peek() {
        match token {
            Token::Op(op, property) => {
                let (root_precedence, root_associativity) =
                    (property.clone().precedence, property.clone().associativity);
                if root_precedence < min_precedence {
                    break;
                }
                tokens.pop(); // consume op
                let op = TokenNode {
                    token: Token::Op(op, property),
                };
                // TODO: impl error handling
                let mut rhs = primary(tokens);
                while let Some(Token::Op(_, property2)) = tokens.peek() {
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
                    rhs = binary_expression(rhs, tokens, precedence)
                }
                lhs = Node::BinExp(BinExpNode {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                });
            }
            _ => break,
        }
    }
    lhs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer;
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
    fn get_suffix(op: &str) -> TokenNode {
        let op = String::from(op);
        let property = lexer::get_property(&op);
        TokenNode {
            token: Token::SuffixOp(op, property),
        }
    }
    fn get_bin_exp(op: &str, lhs: Node, rhs: Node) -> Node {
        Node::BinExp(BinExpNode {
            op: get_op(op),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    fn get_suffix_exp(suffix: &str, lhs: Node) -> Node {
        Node::Suffix(SuffixNode {
            suffix: get_suffix(suffix),
            node: Box::new(lhs),
        })
    }
    fn get_array_index_exp(array: Node, index: Node) -> Node {
        Node::ArrayIndex(ArrayIndexNode {
            array: Box::new(array),
            index: Box::new(index),
        })
    }
    fn get_function_call_exp(identifier: TokenNode, parameters: Node) -> Node {
        Node::FunctionCall(FunctionCallNode {
            identifier,
            parameters: Box::new(parameters),
        })
    }
    fn get_ternary_exp(condition: Node, lhs: Node, rhs: Node) -> Node {
        Node::TernaryExp(TernaryExpNode {
            condition: Box::new(condition),
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
        let expected = get_suffix_exp("++", lhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_index() {
        let actual = run(String::from("a = b[1 + 2]"));
        // expect: a = (b[(1 + 2)])
        let lhs = get_ide("a");
        let index = get_bin_exp("+", get_num(1), get_num(2));
        let array = get_ide("b");
        let rhs = get_array_index_exp(array, index);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call() {
        let actual = run(String::from("a = func(1, 2, 3) + 5"));
        let lhs = get_ide("a");
        let identifier = TokenNode {
            token: Token::Ide("func".to_string()),
        };
        let parameters = get_bin_exp(",", get_num(1), get_num(2));
        let parameters = get_bin_exp(",", parameters, get_num(3));
        let function_call = get_function_call_exp(identifier, parameters);
        let rhs = get_bin_exp("+", function_call, get_num(5));
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_without_args() {
        let actual = run(String::from("a = func() + 5"));
        let lhs = get_ide("a");
        let identifier = TokenNode {
            token: Token::Ide("func".to_string()),
        };
        let parameters = Node::Empty;
        let function_call = get_function_call_exp(identifier, parameters);
        let rhs = get_bin_exp("+", function_call, get_num(5));
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_commma() {
        let actual = run(String::from("a = 1 + 2 * 3, 4 + 5"));
        let lhs = get_bin_exp("*", get_num(2), get_num(3));
        let lhs = get_bin_exp("+", get_num(1), lhs);
        let lhs = get_bin_exp("=", get_ide("a"), lhs);
        let rhs = get_bin_exp("+", get_num(4), get_num(5));
        let expected = get_bin_exp(",", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ternary() {
        let actual = run(String::from("a = 1 ? 10 * 20 + 30 : b++"));
        let lhs = get_ide("a");
        let condition = get_num(1);
        let ternary_lhs = get_bin_exp("*", get_num(10), get_num(20));
        let ternary_lhs = get_bin_exp("+", ternary_lhs, get_num(30));
        let ternary_rhs = get_suffix_exp("++", get_ide("b"));
        let rhs = get_ternary_exp(condition, ternary_lhs, ternary_rhs);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all() {
        let actual = run(String::from("a = b = 1 + 2 * 3++ + 4"));
        // expect: a = (b = ((1 + (2 * 3)) + 4))
        let lhs = get_ide("a");
        let rhs_with_suffix = get_suffix_exp("++", get_num(3));
        let rhs0 = get_bin_exp("*", get_num(2), rhs_with_suffix);
        let rhs1 = get_bin_exp("+", get_num(1), rhs0);
        let rhs2 = get_bin_exp("+", rhs1, get_num(4));
        let rhs = get_bin_exp("=", get_ide("b"), rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }
}
