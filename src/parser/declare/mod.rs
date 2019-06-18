use std::collections::VecDeque;

use crate::lexer::token::*;
use crate::parser::expression::node::*;
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
    pub parameters: VecDeque<DeclareVariableNode>,
    pub statements: VecDeque<StatementNode>,
}
impl FunctionNode {
    pub fn new(tokens: &mut Tokens) -> FunctionNode {
        let return_type = match tokens.pop().unwrap() {
            Token::Type(val, _) => val,
            _ => panic!(),
        };
        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val, _) => val,
            _ => panic!(),
        }; // consume main
        tokens.pop(); // consume (

        let mut parameters: VecDeque<DeclareVariableNode> = VecDeque::new();
        loop {
            if let Some(Token::ParenE(_)) = tokens.peek() {
                tokens.pop(); // consume )
                break;
            }
            let declare_variable_node = DeclareVariableNode::new(tokens);
            parameters.push_back(declare_variable_node);
            if let Some(Token::Op(op, _)) = tokens.peek() {
                if op == "," {
                    tokens.pop(); // consume ,
                }
            }
        }
        tokens.pop(); // consume {

        let mut statements: VecDeque<StatementNode> = VecDeque::new();
        loop {
            if let Some(Token::CurlyE(_)) = tokens.peek() {
                tokens.pop(); // consume }
                break;
            }
            let statement = StatementNode::new(tokens);
            statements.push_back(statement);
        }

        FunctionNode {
            identifier,
            return_type,
            parameters,
            statements,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareVariableNode {
    pub value_type: BasicType,
    pub identifier: String,
    pub initialize_expression: Option<ExpressionNode>,
}
impl DeclareVariableNode {
    pub fn new(tokens: &mut Tokens) -> DeclareVariableNode {
        let mut value_type = match tokens.pop().unwrap() {
            Token::Type(val, _) => val,
            _ => panic!(),
        };
        if let Some(Token::Op(op, _)) = tokens.peek() {
            if op == "*" {
                value_type = BasicType::Pointer(Box::new(value_type));
                tokens.pop();
            }
        }
        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val, _) => val,
            _ => panic!(),
        };

        let mut array_size_vec: Vec<u32> = Vec::new();
        while let Some(Token::SuffixOp(op, _)) = tokens.peek() {
            if op == "[" {
                tokens.pop(); // consume [
                let num = match tokens.pop().unwrap() {
                    Token::IntNum(num, _) => num.parse().unwrap(),
                    _ => panic!(),
                };
                array_size_vec.push(num);
                tokens.pop(); // consume ]
            } else {
                break;
            }
        }
        while let Some(size) = array_size_vec.pop() {
            value_type = BasicType::Array(Box::new(value_type), size);
        }

        let mut initialize_expression = None;
        if let Some(Token::Op(op, _)) = tokens.peek() {
            if op == "=" {
                tokens.pop();
                initialize_expression = Some(ExpressionNode::new(tokens));
            }
        }
        DeclareVariableNode {
            value_type,
            identifier,
            initialize_expression,
        }
    }
}
