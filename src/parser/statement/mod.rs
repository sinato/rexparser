use crate::lexer::token::{Token, Tokens};

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub val: usize,
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        tokens.pop(); // consume return
        let val: Token = tokens.pop().unwrap();
        let val: usize = match val {
            Token::Num(val) => val as usize,
            _ => panic!(),
        };
        StatementNode { val }
    }
}
