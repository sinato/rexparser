use crate::token::{Token, Tokens};

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
impl BinExpNode {}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenNode {
    pub token: Token,
}
impl TokenNode {
    pub fn from_consume(tokens: &mut Tokens) -> TokenNode {
        match tokens.pop() {
            Some(token) => match token {
                Token::Ide(_) | Token::Num(_) => TokenNode { token },
                _ => panic!("Expect a number or an identifier.".to_string()),
            },
            None => panic!("Expect a number or an identifier, but there is no token.".to_string()),
        }
    }
}
