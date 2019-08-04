use crate::lexer::token::*;
use crate::parser::expression::*;
use crate::parser::statement::*;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareNode {
    Function(FunctionNode),
    Variable(DeclareStatementNode),
}
impl DeclareNode {
    pub fn new(tokens: &mut Tokens) -> DeclareNode {
        let mut cloned_token = tokens.clone();
        cloned_token.pop(); //consume type token
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

fn to_pointer_value(value_string: String) -> String {
    value_string + "*"
}
fn to_array_value(value_string: String, size: u32) -> String {
    value_string + "[" + &size.to_string() + "]"
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub identifier: String,
    pub return_type: String,
    pub parameters: VecDeque<DeclareVariableNode>,
    pub statements: Option<VecDeque<StatementNode>>,
    pub is_extern: bool,
    pub is_var_args: bool,
}
impl FunctionNode {
    pub fn new(tokens: &mut Tokens) -> FunctionNode {
        let is_extern = if let Some(Token::Extern(_)) = tokens.peek() {
            tokens.pop();
            true
        } else {
            false
        };

        let return_type = match tokens.pop().unwrap() {
            Token::Ide(val, _) => val,
            _ => panic!(),
        };
        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val, _) => val,
            _ => panic!(),
        }; // consume function name
        tokens.pop(); // consume (

        let mut parameters: VecDeque<DeclareVariableNode> = VecDeque::new();
        let is_var_args = loop {
            match tokens.peek() {
                Some(token) => match token {
                    Token::ParenE(_) => {
                        tokens.pop(); // consume )
                        break false;
                    }
                    Token::Va(_) => {
                        tokens.pop(); // consume ...
                        tokens.pop(); // consume );
                        break true;
                    }
                    _ => (),
                },
                None => panic!("unexpected"),
            }
            let declare_variable_node = DeclareVariableNode::new(tokens, true, None);
            parameters.push_back(declare_variable_node);
            if let Some(Token::Op(op, _)) = tokens.peek() {
                if op == "," {
                    tokens.pop(); // consume ,
                }
            }
        };
        let statements = match tokens.pop() {
            Some(token) => match token {
                Token::CurlyS(_) => {
                    let mut statements: VecDeque<StatementNode> = VecDeque::new();
                    loop {
                        if let Some(Token::CurlyE(_)) = tokens.peek() {
                            tokens.pop(); // consume }
                            break;
                        }
                        let statement = StatementNode::new(tokens);
                        statements.push_back(statement);
                    }
                    Some(statements)
                }
                Token::Semi(_) => None,
                _ => panic!("unexpected"),
            },
            None => panic!("unexpected"),
        };
        FunctionNode {
            identifier,
            return_type,
            parameters,
            statements,
            is_extern,
            is_var_args,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareVariableNode {
    pub value_type: String,
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
                identifier
            }
            Token::Enum(_) => {
                if let Some(Token::Ide(_, _)) = tokens.peek() {
                    tokens.pop();
                }
                String::from("int")
            }
            Token::Ide(type_string, _) => type_string,
            _ => panic!(),
        };

        if let Some(Token::Op(op, _)) = tokens.peek() {
            if op == "*" {
                value_type = to_pointer_value(value_type);
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

pub fn get_array_type_at_function_declare(value_type: String, tokens: &mut Tokens) -> String {
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
    value_type = to_pointer_value(value_type);
    value_type = get_array_type(value_type, tokens);
    value_type
}

fn get_array_type(value_type: String, tokens: &mut Tokens) -> String {
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
        value_type = to_array_value(value_type, size);
    }
    value_type
}
