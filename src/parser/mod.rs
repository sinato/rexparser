pub mod declare;
pub mod expression;
pub mod statement;

use crate::lexer::token::Tokens;
use crate::parser::declare::DeclareNode;

pub fn parser(tokens: &mut Tokens) -> DeclareNode {
    DeclareNode::new(tokens)
}
