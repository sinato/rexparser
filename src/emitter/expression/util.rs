use inkwell::values::PointerValue;

use crate::emitter::expression::*;
use crate::emitter::*;
use crate::lexer::token::*;
use crate::parser::expression::node::*;

pub fn emit_expression_as_pointer(
    emitter: &mut Emitter,
    node: ExpressionNode,
) -> (PointerValue, BasicType, String) {
    match node {
        ExpressionNode::Token(node) => match node.token {
            Token::Ide(identifier) => match emitter.environment.get(&identifier) {
                Some((alloca, variable_type)) => (alloca, variable_type, identifier),
                None => panic!(format!("use of undeclared identifier {}", identifier)),
            },
            _ => panic!(),
        },
        ExpressionNode::ArrayIndex(node) => {
            let array: ExpressionNode = *node.array;
            let index: ExpressionNode = *node.index;

            let index = match emit_expression(emitter, index) {
                Value::Int(val) => val,
                _ => panic!(),
            };
            let (array_alloca, _, identifier) = emit_expression_as_pointer(emitter, array);
            let alloca = unsafe {
                emitter.builder.build_gep(
                    array_alloca,
                    &[emitter.context.i32_type().const_int(0, false), index],
                    "",
                )
            };
            (alloca, BasicType::Int, identifier)
        }
        _ => panic!(),
    }
}

pub fn emit_equal_expression(
    emitter: &mut Emitter,
    alloca: PointerValue,
    alloca_type: BasicType,
    val: Value,
) -> Value {
    match alloca_type {
        BasicType::Int => match val {
            Value::Int(val) => {
                emitter.builder.build_store(alloca, val);
                Value::Int(val)
            }
            Value::Float(_val) => panic!("TODO"),
            _ => panic!("TODO"),
        },
        BasicType::Float => match val {
            Value::Int(_val) => panic!("TODO"),
            Value::Float(val) => {
                emitter.builder.build_store(alloca, val);
                Value::Float(val)
            }
            _ => panic!("TODO"),
        },
        BasicType::Pointer(boxed_type) => match *boxed_type {
            BasicType::Int => match val {
                Value::Pointer(val, val_type) => match val_type {
                    BasicType::Int => {
                        emitter.builder.build_store(alloca, val);
                        Value::Pointer(val, val_type)
                    }
                    _ => panic!("TODO"),
                },
                Value::Array(val, val_type, size) => match val_type {
                    BasicType::Int => {
                        emitter.builder.build_store(alloca, val);
                        Value::Array(val, val_type, size)
                    }
                    _ => panic!("TODO"),
                },
                _ => panic!("TODO"),
            },
            _ => panic!("TODO"),
        },
        _ => panic!(),
    }
}

pub fn emit_comma_as_parameters(emitter: &mut Emitter, node: ExpressionNode) -> Vec<Value> {
    match node {
        ExpressionNode::BinExp(node) => match node.clone().op.token {
            Token::Op(op, _) => match op.as_ref() {
                "," => {
                    let mut lhs = emit_comma_as_parameters(emitter, *node.lhs);
                    let rhs = emit_comma_as_parameters(emitter, *node.rhs);
                    lhs.extend(rhs.iter().cloned());
                    lhs
                }
                _ => vec![emit_bin_exp(emitter, node)],
            },
            _ => panic!(),
        },
        ExpressionNode::Token(node) => vec![emit_token(emitter, node)],
        ExpressionNode::Empty => vec![],
        _ => panic!(),
    }
}
