use std::collections::VecDeque;

use crate::lexer::token::*;
use crate::parser::expression::node::*;
use crate::parser::statement::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareNode {
    Function(FunctionNode),
    Variable(DeclareStatementNode),
}
impl DeclareNode {
    pub fn new(tokens: &mut Tokens) -> DeclareNode {
        let mut cloned_token = tokens.clone();
        cloned_token.pop(); // consume type token
        cloned_token.pop(); // consume identifier
        match cloned_token.pop() {
            Some(token) => match token {
                Token::SuffixOp(_, _) => DeclareNode::Function(FunctionNode::new(tokens)),
                _ => DeclareNode::Variable(DeclareStatementNode::new(tokens)),
            },
            None => panic!(),
        }
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
            let declare_variable_node = DeclareVariableNode::new(tokens, true, None);
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
    pub fn new(
        tokens: &mut Tokens,
        is_function_declare: bool,
        break_op: Option<String>,
    ) -> DeclareVariableNode {
        let mut value_type = match tokens.pop().unwrap() {
            Token::Struct(_) => {
                let identifier = match tokens.pop().unwrap() {
                    Token::Ide(val, _) => val,
                    _ => panic!("unexpected"),
                };
                BasicType::Struct(identifier)
            }
            Token::Enum(_) => {
                if let Some(Token::Ide(_, _)) = tokens.peek() {
                    tokens.pop();
                }
                BasicType::Int
            }
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

        if let Some(Token::SuffixOp(_, _)) = tokens.peek() {
            if is_function_declare {
                value_type = get_array_type_at_function_declare(value_type, tokens);
            } else {
                value_type = get_array_type(value_type, tokens);
            }
        }
        let mut initialize_expression = None;
        if let Some(Token::Op(op, _)) = tokens.peek() {
            if op == "=" {
                tokens.pop();
                initialize_expression = Some(ExpressionNode::new(tokens, break_op));
            }
        }
        DeclareVariableNode {
            value_type,
            identifier,
            initialize_expression,
        }
    }
}

pub fn get_array_type_at_function_declare(value_type: BasicType, tokens: &mut Tokens) -> BasicType {
    let mut value_type = value_type;
    if let Some(Token::SuffixOp(op, _)) = tokens.peek() {
        if op == "[" {
            tokens.pop(); // consume [
            if let Some(Token::SquareE(_)) = tokens.peek() {
                tokens.pop(); // consume ]
            } else {
                tokens.pop();
                tokens.pop(); // consume ]
            }
        }
    }
    value_type = get_array_type(value_type, tokens);
    value_type = BasicType::Pointer(Box::new(value_type));
    value_type
}

fn get_array_type(value_type: BasicType, tokens: &mut Tokens) -> BasicType {
    let mut value_type = value_type;
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
    value_type
}
