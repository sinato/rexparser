use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BinExp(BinExpNode),
    Suffix(SuffixNode),
    Token(TokenNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinExpNode {
    pub op: TokenNode,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
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
        match self.token.clone() {
            Token::Op(op, _) => write!(f, "{}", op),
            Token::SuffixOp(suffix, _) => write!(f, "{}", suffix),
            Token::Ide(ide) => write!(f, "{}", ide),
            Token::Num(num) => write!(f, "{}", num),
        }
    }
}
