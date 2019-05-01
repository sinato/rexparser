use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BinExp(BinExpNode),
    TernaryExp(TernaryExpNode),
    Suffix(SuffixNode),
    ArrayIndex(ArrayIndexNode),
    FunctionCall(FunctionCallNode),
    Token(TokenNode),
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinExpNode {
    pub op: TokenNode,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpNode {
    pub condition: Box<Node>,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
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
pub struct SuffixNode {
    pub suffix: TokenNode,
    pub node: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenNode {
    pub token: Token,
}
impl std::fmt::Display for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
