use crate::lexer::token::Tokens;
use crate::parser::expression::node::ExpressionNode;
use crate::parser::expression::parser::toplevel;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Return(ReturnStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        StatementNode::Return(ReturnStatementNode::new(tokens))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatementNode {
    pub expression: ExpressionNode,
}
impl ReturnStatementNode {
    pub fn new(tokens: &mut Tokens) -> ReturnStatementNode {
        tokens.pop(); // consume return
        let expression = toplevel(tokens);
        ReturnStatementNode { expression }
    }
}
