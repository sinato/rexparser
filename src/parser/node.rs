use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}
impl Nodes {
    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.reverse();
        let node = self.nodes.pop();
        self.nodes.reverse();
        node
    }
    pub fn peek(&self) -> Option<Node> {
        let mut nodes = self.clone().nodes;
        nodes.reverse();
        let node = nodes.pop();
        nodes.reverse();
        node
    }
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
