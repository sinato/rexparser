use crate::lexer::token::{Associativity, Token, Tokens};

#[derive(Debug, PartialEq, Clone)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BinExp(BinExpNode),
    TernaryExp(TernaryExpNode),
    Prefix(PrefixNode),
    Suffix(SuffixNode),
    ArrayIndex(ArrayIndexNode),
    FunctionCall(FunctionCallNode),
    Token(TokenNode),
    Empty,
}
impl Node {
    pub fn new(tokens: &mut Tokens) -> Node {
        let lhs = Node::new_with_prefix(tokens);
        let lhs = Node::new_with_suffix(lhs, tokens);
        lhs
    }
    fn new_with_prefix(tokens: &mut Tokens) -> Node {
        match tokens.peek() {
            Some(token) => match token {
                Token::Ide(_) | Token::Num(_) => TokenNode::new(tokens),
                Token::PrefixOp(_) => PrefixNode::new(tokens),
                Token::Op(op, _property) => match op.as_ref() {
                    // treat as a sing operator
                    "+" | "-" => {
                        tokens.pop(); // consume "+" | "-"
                        let node = Node::new(tokens);
                        Node::Prefix(PrefixNode {
                            prefix: TokenNode {
                                token: Token::PrefixOp(op),
                            },
                            node: Box::new(node),
                        })
                    }
                    _ => panic!(),
                },
                Token::SuffixOp(suffix) => match suffix.as_ref() {
                    // treat as a parenthesis expression
                    "(" => {
                        tokens.pop(); // consume "("
                        let node = BinExpNode::new(tokens);
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
    fn new_with_suffix(lhs: Node, tokens: &mut Tokens) -> Node {
        match tokens.peek() {
            Some(token) => match token {
                Token::SuffixOp(_) => {
                    let mut node = SuffixNode::new(lhs, tokens);
                    while let Some(Token::SuffixOp(_)) = tokens.peek() {
                        node = SuffixNode::new(node, tokens);
                    }
                    node
                }
                Token::Question => TernaryExpNode::new(lhs, tokens),
                _ => lhs,
            },
            None => lhs,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinExpNode {
    pub op: TokenNode,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}
impl BinExpNode {
    pub fn new(tokens: &mut Tokens) -> Node {
        let lhs = Node::new(tokens);
        BinExpNode::binary_expression(lhs, tokens, 0)
    }
    fn binary_expression(mut lhs: Node, tokens: &mut Tokens, min_precedence: u32) -> Node {
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
                    let mut rhs = Node::new(tokens);
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
                        rhs = BinExpNode::binary_expression(rhs, tokens, precedence)
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpNode {
    pub condition: Box<Node>,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}
impl TernaryExpNode {
    pub fn new(lhs: Node, tokens: &mut Tokens) -> Node {
        let condition = lhs.clone();
        let _question = tokens.pop();
        let ternary_lhs = BinExpNode::new(tokens);
        let _colon = tokens.pop();
        let ternary_rhs = BinExpNode::new(tokens);
        Node::TernaryExp(TernaryExpNode {
            condition: Box::new(condition),
            lhs: Box::new(ternary_lhs),
            rhs: Box::new(ternary_rhs),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayIndexNode {
    pub array: Box<Node>,
    pub index: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub identifier: TokenNode,
    pub parameters: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixNode {
    pub prefix: TokenNode,
    pub node: Box<Node>,
}
impl PrefixNode {
    pub fn new(tokens: &mut Tokens) -> Node {
        let token = tokens.pop().unwrap();
        let node = Node::new(tokens);
        Node::Prefix(PrefixNode {
            prefix: TokenNode { token },
            node: Box::new(node),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuffixNode {
    pub suffix: TokenNode,
    pub node: Box<Node>,
}
impl SuffixNode {
    fn new(lhs: Node, tokens: &mut Tokens) -> Node {
        match tokens.pop().unwrap() {
            Token::SuffixOp(suffix) => match suffix.as_ref() {
                "++" => Node::Suffix(SuffixNode {
                    suffix: TokenNode {
                        token: Token::SuffixOp(suffix),
                    },
                    node: Box::new(lhs),
                }),
                "[" => {
                    let index = BinExpNode::new(tokens);
                    let array = Node::ArrayIndex(ArrayIndexNode {
                        array: Box::new(lhs),
                        index: Box::new(index),
                    });
                    tokens.pop(); // consume "["
                    array
                }
                "(" => {
                    if let Node::Token(token_node) = lhs {
                        let parameters = match tokens.peek() {
                            Some(token) => match token {
                                Token::ParenE => Box::new(Node::Empty),
                                _ => Box::new(BinExpNode::new(tokens)),
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenNode {
    pub token: Token,
}
impl TokenNode {
    pub fn new(tokens: &mut Tokens) -> Node {
        let token = tokens.pop().unwrap();
        Node::Token(TokenNode { token })
    }
}
impl std::fmt::Display for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
