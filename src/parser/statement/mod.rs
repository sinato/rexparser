use crate::lexer::token::Tokens;
use crate::parser::expression::node::Node as ExpNode;
use crate::parser::expression::parser::toplevel;

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub expression: ExpNode,
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        tokens.pop(); // consume return
        let expression = toplevel(tokens);
        StatementNode { expression }
    }
}
