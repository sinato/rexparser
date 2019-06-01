use crate::lexer::token::Tokens;
use crate::parser::expression::node::ExpressionNode;
use crate::parser::expression::parser::toplevel;

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub expression: ExpressionNode,
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        tokens.pop(); // consume return
        let expression = toplevel(tokens);
        StatementNode { expression }
    }
}
