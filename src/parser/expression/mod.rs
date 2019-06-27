use crate::lexer::token::Property;
use crate::lexer::token::{Associativity, Token, Tokens};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNodes {
    pub nodes: Vec<ExpressionNode>,
}

pub fn get_property(op: &String) -> Property {
    let mut map = HashMap::new();
    map.insert("=", (2, Associativity::Right));
    map.insert("+=", (2, Associativity::Right));
    map.insert("||", (4, Associativity::Left));
    map.insert("&&", (5, Associativity::Left));
    map.insert("==", (9, Associativity::Left));
    map.insert(">", (10, Associativity::Left));
    map.insert("<", (10, Associativity::Left));
    map.insert("+", (12, Associativity::Left));
    map.insert("-", (12, Associativity::Left));
    map.insert("*", (13, Associativity::Left));
    map.insert("[", (16, Associativity::Left));
    map.insert("(", (16, Associativity::Left));
    map.insert(".", (16, Associativity::Left));
    map.insert(",", (1, Associativity::Left));
    let op: &str = &op;
    let (precedence, associativity): (u32, Associativity) = map[op].clone();
    Property {
        precedence,
        associativity,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    BinExp(BinExpNode),
    TernaryExp(TernaryExpNode),
    Prefix(PrefixNode),
    Suffix(SuffixNode),
    ArrayIndex(ArrayIndexNode),
    FunctionCall(FunctionCallNode),
    Access(AccessNode),
    Token(TokenNode),
    Empty,
}
impl ExpressionNode {
    pub fn new(tokens: &mut Tokens, break_op: Option<String>) -> ExpressionNode {
        BinExpNode::new(tokens, break_op)
    }
    pub fn new_node(tokens: &mut Tokens) -> ExpressionNode {
        let lhs = ExpressionNode::new_with_prefix(tokens);
        let lhs = ExpressionNode::new_with_suffix(lhs, tokens);
        lhs
    }
    fn new_with_prefix(tokens: &mut Tokens) -> ExpressionNode {
        match tokens.peek() {
            Some(token) => match token {
                Token::Ide(_, _) | Token::IntNum(_, _) | Token::FloatNum(_, _) => {
                    TokenNode::new(tokens)
                }
                Token::PrefixOp(_, _) => PrefixNode::new(tokens),
                Token::Op(op, debug_info) => match op.as_ref() {
                    // treat as an operator
                    "+" | "-" | "*" => {
                        tokens.pop(); // consume "+" | "-"
                        let node = ExpressionNode::new_node(tokens);
                        ExpressionNode::Prefix(PrefixNode {
                            prefix: TokenNode {
                                token: Token::PrefixOp(op, debug_info),
                            },
                            node: Box::new(node),
                        })
                    }
                    _ => panic!(),
                },
                Token::SuffixOp(suffix, _) => match suffix.as_ref() {
                    // treat as a parenthesis expression
                    "(" => {
                        tokens.pop(); // consume "("
                        let node = BinExpNode::new(tokens, None);
                        tokens.pop(); // consume ")"
                        node
                    }
                    _ => panic!(),
                },
                _ => panic!(format!("Expect a primary token, but this is {:?}", token)),
            },
            None => panic!(),
        }
    }
    fn new_with_suffix(lhs: ExpressionNode, tokens: &mut Tokens) -> ExpressionNode {
        match tokens.peek() {
            Some(token) => match token {
                Token::SuffixOp(_, _) => {
                    let mut node = SuffixNode::new(lhs, tokens);
                    while let Some(Token::SuffixOp(_, _)) = tokens.peek() {
                        node = SuffixNode::new(node, tokens);
                    }
                    node
                }
                Token::Question(_) => TernaryExpNode::new(lhs, tokens),
                _ => lhs,
            },
            None => lhs,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinExpNode {
    pub op: TokenNode,
    pub lhs: Box<ExpressionNode>,
    pub rhs: Box<ExpressionNode>,
}
impl BinExpNode {
    pub fn new(tokens: &mut Tokens, break_op: Option<String>) -> ExpressionNode {
        let lhs = ExpressionNode::new_node(tokens);
        BinExpNode::binary_expression(lhs, tokens, 0, break_op)
    }
    fn binary_expression(
        mut lhs: ExpressionNode,
        tokens: &mut Tokens,
        min_precedence: u32,
        break_op: Option<String>,
    ) -> ExpressionNode {
        while let Some(token) = tokens.peek() {
            match token {
                Token::Op(op, debug_info) => {
                    if let Some(break_op) = break_op.clone() {
                        if break_op == op {
                            break;
                        }
                    }

                    let property = get_property(&op);
                    let (root_precedence, root_associativity) =
                        (property.clone().precedence, property.clone().associativity);
                    if root_precedence < min_precedence {
                        break;
                    }
                    tokens.pop(); // consume op
                    let op = TokenNode {
                        token: Token::Op(op, debug_info),
                    };
                    // TODO: impl error handling
                    let mut rhs = ExpressionNode::new_node(tokens);
                    while let Some(Token::Op(op2, _)) = tokens.peek() {
                        let property2 = get_property(&op2);
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
                        rhs =
                            BinExpNode::binary_expression(rhs, tokens, precedence, break_op.clone())
                    }
                    lhs = ExpressionNode::BinExp(BinExpNode {
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpNode {
    pub condition: Box<ExpressionNode>,
    pub lhs: Box<ExpressionNode>,
    pub rhs: Box<ExpressionNode>,
}
impl TernaryExpNode {
    pub fn new(lhs: ExpressionNode, tokens: &mut Tokens) -> ExpressionNode {
        let condition = lhs.clone();
        let _question = tokens.pop();
        let ternary_lhs = BinExpNode::new(tokens, None);
        let _colon = tokens.pop();
        let ternary_rhs = BinExpNode::new(tokens, None);
        ExpressionNode::TernaryExp(TernaryExpNode {
            condition: Box::new(condition),
            lhs: Box::new(ternary_lhs),
            rhs: Box::new(ternary_rhs),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayIndexNode {
    pub array: Box<ExpressionNode>,
    pub index: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub identifier: TokenNode,
    pub parameters: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixNode {
    pub prefix: TokenNode,
    pub node: Box<ExpressionNode>,
}
impl PrefixNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionNode {
        let token = tokens.pop().unwrap();
        let node = ExpressionNode::new_node(tokens);
        ExpressionNode::Prefix(PrefixNode {
            prefix: TokenNode { token },
            node: Box::new(node),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuffixNode {
    pub suffix: TokenNode,
    pub node: Box<ExpressionNode>,
}
impl SuffixNode {
    fn new(lhs: ExpressionNode, tokens: &mut Tokens) -> ExpressionNode {
        match tokens.pop().unwrap() {
            Token::SuffixOp(suffix, debug_info) => match suffix.as_ref() {
                "++" => ExpressionNode::Suffix(SuffixNode {
                    suffix: TokenNode {
                        token: Token::SuffixOp(suffix, debug_info),
                    },
                    node: Box::new(lhs),
                }),
                "[" => {
                    let index = BinExpNode::new(tokens, None);
                    let array = ExpressionNode::ArrayIndex(ArrayIndexNode {
                        array: Box::new(lhs),
                        index: Box::new(index),
                    });
                    tokens.pop(); // consume "["
                    array
                }
                "(" => {
                    if let ExpressionNode::Token(token_node) = lhs {
                        let parameters = match tokens.peek() {
                            Some(token) => match token {
                                Token::ParenE(_) => Box::new(ExpressionNode::Empty),
                                _ => Box::new(BinExpNode::new(tokens, None)),
                            },
                            None => panic!(),
                        };
                        tokens.pop(); // consume ParanE TODO: impl error handling
                        ExpressionNode::FunctionCall(FunctionCallNode {
                            identifier: token_node,
                            parameters,
                        })
                    } else {
                        panic!("Expect a token node as lhs.")
                    }
                }
                "." => {
                    let access_identifier = tokens.pop().unwrap();
                    ExpressionNode::Access(AccessNode {
                        access_identifier,
                        node: Box::new(lhs),
                    })
                }
                _ => panic!(),
            },
            _ => panic!("Expect a suffix operator."),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AccessNode {
    pub access_identifier: Token,
    pub node: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenNode {
    pub token: Token,
}
impl TokenNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionNode {
        let token = tokens.pop().unwrap();
        ExpressionNode::Token(TokenNode { token })
    }
}
