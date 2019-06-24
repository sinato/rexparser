use crate::lexer::token::*;
use crate::parser::declare::*;
use crate::parser::expression::node::ExpressionNode;
use crate::parser::*;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Expression(ExpressionStatementNode),
    Return(ReturnStatementNode),
    Declare(DeclareStatementNode),
    Struct(StructStatementNode),
    Compound(CompoundStatementNode),
    If(IfStatementNode),
    For(ForStatementNode),
    While(WhileStatementNode),
    Break(BreakStatementNode),
    Continue(ContinueStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek().unwrap() {
            Token::Type(_, _) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
            Token::Struct(_) => StatementNode::Struct(StructStatementNode::new(tokens)),
            Token::Return(_) => StatementNode::Return(ReturnStatementNode::new(tokens)),
            Token::CurlyS(_) => StatementNode::Compound(CompoundStatementNode::new(tokens)),
            Token::If(_) => StatementNode::If(IfStatementNode::new(tokens)),
            Token::While(_) => StatementNode::While(WhileStatementNode::new(tokens)),
            Token::Break(_) => StatementNode::Break(BreakStatementNode::new(tokens)),
            Token::Continue(_) => StatementNode::Continue(ContinueStatementNode::new(tokens)),
            Token::For(_) => StatementNode::For(ForStatementNode::new(tokens)),
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
            if let Some(Token::CurlyE(_)) = tokens.peek() {
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
            Token::Semi(_) => (),
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
            Token::Semi(_) => (),
            _ => panic!(),
        };
        ReturnStatementNode { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructStatementNode {
    Declare(DeclareStatementNode),
    Definition(StructDefinitionNode),
}
impl StructStatementNode {
    pub fn new(tokens: &mut Tokens) -> StructStatementNode {
        let mut cloned_tokens = tokens.clone();
        cloned_tokens.pop(); // consume "struct"
        cloned_tokens.pop(); // consume identifier
        match cloned_tokens.pop().unwrap() {
            Token::CurlyS(_) => StructStatementNode::Definition(StructDefinitionNode::new(tokens)),
            _ => StructStatementNode::Declare(DeclareStatementNode::new(tokens)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatementNode {
    pub declare_variable_node: DeclareVariableNode,
}
impl DeclareStatementNode {
    pub fn new(tokens: &mut Tokens) -> DeclareStatementNode {
        let declare_variable_node = DeclareVariableNode::new(tokens, false);
        match tokens.pop().unwrap() {
            Token::Semi(_) => (),
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
    pub block: Box<StatementNode>,
    pub else_block: Option<Box<StatementNode>>,
}
impl IfStatementNode {
    pub fn new(tokens: &mut Tokens) -> IfStatementNode {
        tokens.pop(); // consume if
        tokens.pop(); // consume (
        let condition_expression = ExpressionNode::new(tokens);
        tokens.pop(); // consume )
        let block = Box::new(StatementNode::new(tokens));
        let else_block = match tokens.peek() {
            Some(token) => match token {
                Token::Else(_) => {
                    tokens.pop(); // consume else
                    Some(Box::new(StatementNode::new(tokens)))
                }
                _ => None,
            },
            None => None,
        };
        IfStatementNode {
            condition_expression,
            block,
            else_block,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatementNode {
    pub condition_expression: ExpressionNode,
    pub block: Box<StatementNode>,
}
impl WhileStatementNode {
    pub fn new(tokens: &mut Tokens) -> WhileStatementNode {
        tokens.pop(); // consume while
        tokens.pop(); // consume (
        let condition_expression = ExpressionNode::new(tokens);
        tokens.pop(); // consume )
        let block = Box::new(StatementNode::new(tokens));
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
pub struct ContinueStatementNode {}
impl ContinueStatementNode {
    pub fn new(tokens: &mut Tokens) -> ContinueStatementNode {
        tokens.pop(); // consume continue
        tokens.pop(); // consume ;
        ContinueStatementNode {}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatementNode {
    pub first_statement: Box<StatementNode>,
    pub condition_expression: ExpressionNode,
    pub loop_expression: ExpressionNode,
    pub block: Box<StatementNode>,
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
        let block = Box::new(StatementNode::new(tokens));
        ForStatementNode {
            first_statement,
            condition_expression,
            loop_expression,
            block,
        }
    }
}
