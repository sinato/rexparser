use crate::lexer::token::*;
use crate::parser::declare::*;
use crate::parser::expression::node::ExpressionNode;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Expression(ExpressionStatementNode),
    Return(ReturnStatementNode),
    Declare(DeclareStatementNode),
    Compound(CompoundStatementNode),
    If(IfStatementNode),
    For(ForStatementNode),
    While(WhileStatementNode),
    Break(BreakStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek().unwrap() {
            Token::Type(_) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
            Token::Return => StatementNode::Return(ReturnStatementNode::new(tokens)),
            Token::CurlyS => StatementNode::Compound(CompoundStatementNode::new(tokens)),
            Token::If => StatementNode::If(IfStatementNode::new(tokens)),
            Token::While => StatementNode::While(WhileStatementNode::new(tokens)),
            Token::Break => StatementNode::Break(BreakStatementNode::new(tokens)),
            Token::For => StatementNode::For(ForStatementNode::new(tokens)),
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
        let expression = ExpressionNode::new(tokens);
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
        let expression = ExpressionNode::new(tokens);
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

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatementNode {
    pub condition_expression: ExpressionNode,
    pub block: CompoundStatementNode,
}
impl IfStatementNode {
    pub fn new(tokens: &mut Tokens) -> IfStatementNode {
        tokens.pop(); // consume if
        tokens.pop(); // consume (
        let condition_expression = ExpressionNode::new(tokens);
        tokens.pop(); // consume )
        let block = CompoundStatementNode::new(tokens);
        IfStatementNode {
            condition_expression,
            block,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatementNode {
    pub condition_expression: ExpressionNode,
    pub block: CompoundStatementNode,
}
impl WhileStatementNode {
    pub fn new(tokens: &mut Tokens) -> WhileStatementNode {
        tokens.pop(); // consume while
        tokens.pop(); // consume (
        let condition_expression = ExpressionNode::new(tokens);
        tokens.pop(); // consume )
        let block = CompoundStatementNode::new(tokens);
        WhileStatementNode {
            condition_expression,
            block,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatementNode {}
impl BreakStatementNode {
    pub fn new(tokens: &mut Tokens) -> BreakStatementNode {
        tokens.pop(); // consume break
        tokens.pop(); // consume ;
        BreakStatementNode {}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatementNode {
    pub first_statement: Box<StatementNode>,
    pub condition_expression: ExpressionNode,
    pub loop_expression: ExpressionNode,
    pub block: CompoundStatementNode,
}
impl ForStatementNode {
    pub fn new(tokens: &mut Tokens) -> ForStatementNode {
        tokens.pop(); // consume for
        tokens.pop(); // consume (
        let first_statement = Box::new(StatementNode::new(tokens));
        let condition_expression = ExpressionNode::new(tokens);
        tokens.pop(); // consume ;
        let loop_expression = ExpressionNode::new(tokens);
        tokens.pop(); // consume )
        let block = CompoundStatementNode::new(tokens);
        ForStatementNode {
            first_statement,
            condition_expression,
            loop_expression,
            block,
        }
    }
}
