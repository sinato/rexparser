pub mod declare;
pub mod expression;
pub mod statement;

use crate::lexer::token::Tokens;
use crate::parser::declare::DeclareNode;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramNode {
    pub declares: VecDeque<DeclareNode>,
}
impl ProgramNode {
    pub fn new(tokens: &mut Tokens) -> ProgramNode {
        let mut declares: VecDeque<DeclareNode> = VecDeque::new();
        while let Some(_) = tokens.peek() {
            let declare = DeclareNode::new(tokens);
            declares.push_back(declare);
        }
        ProgramNode { declares }
    }
}

pub fn parser(tokens: &mut Tokens) -> ProgramNode {
    ProgramNode::new(tokens)
}
