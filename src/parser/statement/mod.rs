use crate::lexer::token::*;
use crate::parser::expression::node::ExpressionNode;
use crate::parser::expression::parser::toplevel;

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
    Expression(ExpressionStatementNode),
    Return(ReturnStatementNode),
    Declare(DeclareStatementNode),
}
impl StatementNode {
    pub fn new(tokens: &mut Tokens) -> StatementNode {
        match tokens.peek().unwrap() {
            Token::Type(_) => StatementNode::Declare(DeclareStatementNode::new(tokens)),
            Token::Return => StatementNode::Return(ReturnStatementNode::new(tokens)),
            _ => StatementNode::Expression(ExpressionStatementNode::new(tokens)),
        }
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
    pub value_type: BasicType,
    pub identifier: String,
    pub initialize_expression: Option<ExpressionNode>,
}
impl DeclareStatementNode {
    pub fn new(tokens: &mut Tokens) -> DeclareStatementNode {
        let mut value_type = match tokens.pop().unwrap() {
            Token::Type(val) => val,
            _ => panic!(),
        };
        if let Some(Token::Op(op, _)) = tokens.peek() {
            if op == "*" {
                value_type = BasicType::Pointer(Box::new(value_type));
                tokens.pop();
            }
        }
        let identifier = match tokens.pop().unwrap() {
            Token::Ide(val) => val,
            _ => panic!(),
        };

        let mut array_size_vec: Vec<u32> = Vec::new();
        while let Some(Token::SuffixOp(op)) = tokens.peek() {
            if op == "[" {
                tokens.pop(); // consume [
                let num = match tokens.pop().unwrap() {
                    Token::IntNum(num) => num.parse().unwrap(),
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
