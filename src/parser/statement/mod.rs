use crate::lexer::token::*;
use crate::parser::expression::node::ExpressionNode;
use crate::parser::expression::parser::toplevel;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Return(ReturnStatementNode),
    Declare(DeclareStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek().unwrap() {
            Token::Type(_) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
            _ => StatementNode::Return(ReturnStatementNode::new(tokens)),
        }
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
        match tokens.pop().unwrap() {
            Token::Semi => (),
            _ => panic!(),
        };
        ReturnStatementNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatementNode {
    pub value_type: BasicType,
    pub identifier: String,
    pub initialize_expression: Option<ExpressionNode>,
}
impl DeclareStatementNode {
    pub fn new(tokens: &mut Tokens) -> DeclareStatementNode {
        let value_type = match tokens.pop().unwrap() {
            Token::Type(val) => val,
            _ => panic!(),
        };
        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val) => val,
            _ => panic!(),
        };
        let initialize_expression = None;
        match tokens.pop().unwrap() {
            Token::Semi => (),
            _ => panic!(),
        };
        DeclareStatementNode {
            value_type,
            identifier,
            initialize_expression,
        }
    }
}
