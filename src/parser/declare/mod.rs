pub mod parser;

use crate::lexer::token::{Token, Tokens};
use crate::parser::statement::StatementNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Function(FunctionNode),
}
impl Node {
    pub fn new(tokens: &mut Tokens) -> Node {
        Node::Function(FunctionNode::new(tokens))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub identifier: String,
    pub statement: StatementNode,
}
impl FunctionNode {
    pub fn new(tokens: &mut Tokens) -> FunctionNode {
        tokens.pop(); // consume int

        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val) => val,
            _ => panic!(),
        }; // consume main
        tokens.pop(); // consume (
        tokens.pop(); // consume {

        let statement = StatementNode::new(tokens); // consume return val;
        tokens.pop(); // consume }

        FunctionNode {
            identifier,
            statement,
        }
    }
}
