pub mod declare;
pub mod expression;
pub mod statement;

use crate::emitter::environment::*;
use crate::lexer::token::*;
use crate::parser::declare::*;
use crate::parser::expression::*;
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

#[derive(Debug, PartialEq, Clone)]
pub struct EnumDefinitionNode {
    pub tag: Option<String>,
    pub enums: Vec<DeclareVariableNode>,
}
impl EnumDefinitionNode {
    pub fn new(tokens: &mut Tokens) -> EnumDefinitionNode {
        tokens.pop(); // consume "enum"
        let tag = if let Some(Token::Ide(ide, _)) = tokens.peek() {
            tokens.pop();
            Some(ide)
        } else {
            None
        };
        tokens.pop(); // consume {

        let mut enums: Vec<DeclareVariableNode> = Vec::new();

        let mut cnt = 0;
        let debug_info = DebugInfo {
            start: 0,
            end: 0,
            s: String::from("dummy token for enum value"),
        };
        let mut pre_init_expression = ExpressionNode::Token(TokenNode {
            token: Token::IntNum("0".to_string(), debug_info.clone()),
        });
        loop {
            if let Some(Token::CurlyE(_)) = tokens.peek() {
                tokens.pop(); // consume }
                tokens.pop(); // consume ;
                break;
            }

            // input dummy token to treat the enum as a variable declare.
            // Ex. GREEN = 10 -> int GREEN = 10
            tokens.reverse();
            tokens.push(Token::Type(BasicType::Int, debug_info.clone()));
            tokens.reverse();

            let mut declare_variable_node =
                DeclareVariableNode::new(tokens, true, Some(String::from(",")));

            match declare_variable_node.clone().initialize_expression {
                Some(init_expression) => pre_init_expression = init_expression,
                None => {
                    if cnt != 0 {
                        if let ExpressionNode::Token(TokenNode {
                            token: Token::IntNum(num_str, _),
                        }) = pre_init_expression.clone()
                        {
                            let num: u64 = num_str.parse().unwrap();
                            pre_init_expression = ExpressionNode::Token(TokenNode {
                                token: Token::IntNum((num + 1).to_string(), debug_info.clone()),
                            });
                        } else {
                            panic!()
                        };
                    }
                    declare_variable_node.initialize_expression = Some(pre_init_expression.clone());
                }
            }
            cnt += 1;

            enums.push(declare_variable_node);
            if let Some(Token::Op(op, _)) = tokens.peek() {
                if op == "," {
                    tokens.pop(); // consume ,
                }
            }
        }

        EnumDefinitionNode { tag, enums }
    }
}
