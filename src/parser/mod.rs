pub mod declare;
pub mod expression;
pub mod statement;

use crate::lexer::token::Tokens;
use crate::parser::declare::Node;

pub fn parser(tokens: &mut Tokens) -> Node {
    Node::new(tokens)
}
