pub mod parser;

use crate::lexer::token::*;
use crate::parser::statement::StatementNode;

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareNode {
    Function(FunctionNode),
}
impl DeclareNode {
    pub fn new(tokens: &mut Tokens) -> DeclareNode {
        DeclareNode::Function(FunctionNode::new(tokens))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub identifier: String,
    pub return_type: BasicType,
    pub statement: StatementNode,
}
impl FunctionNode {
    pub fn new(tokens: &mut Tokens) -> FunctionNode {
        let return_type = match tokens.pop().unwrap() {
            Token::Type(val) => val,
            _ => panic!(),
        };

        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val) => val,
            _ => panic!(),
        }; // consume main
        tokens.pop(); // consume (
        tokens.pop(); // consume )
        tokens.pop(); // consume {

        let statement = StatementNode::new(tokens); // consume return val;
        tokens.pop(); // consume }

        FunctionNode {
            identifier,
            return_type,
            statement,
        }
    }
}
