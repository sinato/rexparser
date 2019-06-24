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
            Token::Ide(identifier, _) => match emitter.environment.get(&identifier) {
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
            let (array_alloca, val_type, identifier) = emit_expression_as_pointer(emitter, array);
            match val_type {
                BasicType::Pointer(boxed_type) => {
                    let array_alloca = emitter
                        .builder
                        .build_load(array_alloca, "arr_pointer")
                        .into_pointer_value();
                    let alloca =
                        unsafe { emitter.builder.build_gep(array_alloca, &[index], "gep") };
                    (alloca, *boxed_type, identifier)
                }
                BasicType::Array(boxed_type, _) => {
                    let const_zero = emitter.context.i32_type().const_int(0, false);
                    let alloca = unsafe {
                        emitter
                            .builder
                            .build_gep(array_alloca, &[const_zero, index], "gep")
                    };
                    (alloca, *boxed_type, identifier)
                }
                _ => panic!(),
            }
        }
        ExpressionNode::Access(node) => {
            let exp: ExpressionNode = *node.node;
            let access_identifier: Token = node.access_identifier;

            let (struct_alloca, variable_type, identifier) =
                emit_expression_as_pointer(emitter, exp);
            match variable_type.clone() {
                BasicType::Struct(struct_identifier) => {
                    let target_struct = match emitter.environment.get_struct(&struct_identifier) {
                        Some(target_struct) => target_struct,
                        None => panic!("undeclared struct"),
                    };
                    let access_identifier = match access_identifier {
                        Token::Ide(identifier, _) => identifier,
                        _ => panic!("unexpected"),
                    };
                    let (member_idx, member_type) = match target_struct.find(&access_identifier) {
                        Some((member_idx, member_type)) => (member_idx, member_type),
                        None => panic!(format!(
                            "no member named '{}' 'struct {}'",
                            access_identifier, struct_identifier
                        )),
                    };
                    let alloca = unsafe {
                        emitter
                            .builder
                            .build_struct_gep(struct_alloca, member_idx, "struct_gep")
                    };
                    (alloca, member_type, identifier)
                }
                _ => panic!(),
            }
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
                Value::Array(val, alloca, val_type, size) => match val_type {
                    BasicType::Int => {
                        emitter.builder.build_store(alloca, val);
                        Value::Array(val, alloca, val_type, size)
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

pub fn emit_compare_expression(
    emitter: &mut Emitter,
    operator: &str,
    lhs: BasicValueEnum,
    rhs: BasicValueEnum,
) -> IntValue {
    let fn_value = match emitter.module.get_function(operator) {
        Some(value) => value,
        None => panic!(format!("call of undeclared function {}", operator)),
    };
    let arguments: Vec<BasicValueEnum> = vec![lhs, rhs];
    let func_call_site = emitter
        .builder
        .build_call(fn_value, &arguments, "callcompare");
    let val = func_call_site
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();
    val
}
