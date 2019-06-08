use crate::lexer::token::*;
use crate::parser::declare::*;
use crate::parser::expression::node::ExpressionNode;
use crate::parser::expression::parser::toplevel;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Expression(ExpressionStatementNode),
    Return(ReturnStatementNode),
    Declare(DeclareStatementNode),
    Compound(CompoundStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek().unwrap() {
            Token::Type(_) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
            Token::Return => StatementNode::Return(ReturnStatementNode::new(tokens)),
            Token::CurlyS => StatementNode::Compound(CompoundStatementNode::new(tokens)),
            _ => StatementNode::Expression(ExpressionStatementNode::new(tokens)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundStatementNode {
    pub statements: VecDeque<StatementNode>,
}
impl CompoundStatementNode {
    pub fn new(tokens: &mut Tokens) -> CompoundStatementNode {
        tokens.pop(); // consume {
        let mut statements: VecDeque<StatementNode> = VecDeque::new();
        loop {
            if let Some(Token::CurlyE) = tokens.peek() {
                tokens.pop();
                break;
            }
            let statement = StatementNode::new(tokens);
            statements.push_back(statement);
        }
        CompoundStatementNode { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatementNode {
    pub expression: ExpressionNode,
}
impl ExpressionStatementNode {
    pub fn new(tokens: &mut Tokens) -> ExpressionStatementNode {
        let expression = toplevel(tokens);
        match tokens.pop().unwrap() {
            Token::Semi => (),
            _ => panic!(),
        };
        ExpressionStatementNode { expression }
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
    pub declare_variable_node: DeclareVariableNode,
}
impl DeclareStatementNode {
    pub fn new(tokens: &mut Tokens) -> DeclareStatementNode {
        let declare_variable_node = DeclareVariableNode::new(tokens);
        match tokens.pop().unwrap() {
            Token::Semi => (),
            _ => panic!(),
        };
        DeclareStatementNode {
            declare_variable_node,
        }
    }
}
