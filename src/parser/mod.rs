pub mod declare;
pub mod expression;
pub mod statement;

use crate::emitter::environment::*;
use crate::lexer::token::Tokens;
use crate::lexer::token::*;
use crate::parser::declare::DeclareNode;
use crate::parser::statement::*;

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

#[derive(Debug, PartialEq, Clone)]
pub struct StructDefinitionNode {
    pub target_struct: Struct,
}
impl StructDefinitionNode {
    pub fn new(tokens: &mut Tokens) -> StructDefinitionNode {
        tokens.pop(); // consume "struct"
        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val, _) => val,
            _ => panic!(),
        }; // get type name
        tokens.pop(); // consume {

        let mut members: Vec<(String, BasicType)> = Vec::new();
        loop {
            if let Some(Token::CurlyE(_)) = tokens.peek() {
                tokens.pop(); // consume }
                tokens.pop(); // consume ;
                break;
            }
            let declare_statement_node = DeclareStatementNode::new(tokens);
            let declare_variable_node = declare_statement_node.declare_variable_node;
            members.push((
                declare_variable_node.identifier,
                declare_variable_node.value_type,
            ));
        }
        let target_struct = Struct {
            identifier,
            members,
        };
        StructDefinitionNode { target_struct }
    }
}
