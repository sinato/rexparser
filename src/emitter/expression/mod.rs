pub mod util;

use crate::emitter::expression::util::*;
use crate::emitter::*;
use crate::lexer::token::*;
use crate::parser::expression::node::*;

use inkwell::values::{BasicValueEnum, PointerValue};

pub fn emit_expression(emitter: &mut Emitter, node: ExpressionNode) -> Value {
    match node {
        ExpressionNode::BinExp(node) => emit_bin_exp(emitter, node),
        ExpressionNode::Token(node) => emit_token(emitter, node),
        ExpressionNode::Prefix(node) => emit_prefix(emitter, node),
        ExpressionNode::Suffix(node) => emit_suffix(emitter, node),
        ExpressionNode::ArrayIndex(node) => emit_array_index(emitter, node),
        ExpressionNode::FunctionCall(node) => emit_function_call(emitter, node),
        _ => panic!(""),
    }
}

fn emit_bin_exp(emitter: &mut Emitter, node: BinExpNode) -> Value {
    let operator = match node.op.token {
        Token::Op(op, _) => op,
        _ => panic!(),
    };
    match operator.as_ref() {
        "=" => {
            let (alloca, alloca_type, _identifier): (PointerValue, BasicType, String) =
                emit_expression_as_pointer(emitter, *node.lhs);
            let val: Value = emit_expression(emitter, *node.rhs);
            emit_equal_expression(emitter, alloca, alloca_type, val)
        }
        "+=" => {
            let (alloca, alloca_type, identifier): (PointerValue, BasicType, String) =
                emit_expression_as_pointer(emitter, *node.lhs);
            match alloca_type {
                BasicType::Int => {
                    let val = emitter
                        .builder
                        .build_load(alloca, &identifier)
                        .into_int_value();

                    let add_val = match emit_expression(emitter, *node.rhs) {
                        Value::Int(value) => value,
                        _ => panic!(),
                    };
                    let added_val = emitter.builder.build_int_add(val, add_val, "add");
                    emitter.builder.build_store(alloca, added_val);
                    Value::Int(added_val)
                }
                _ => panic!(),
            }
        }
        _ => {
            let lhs = emit_expression(emitter, *node.lhs);
            let rhs = emit_expression(emitter, *node.rhs);
            match lhs {
                Value::Int(lhs) => match rhs {
                    Value::Int(rhs) => {
                        let val = match operator.as_ref() {
                            "+" => emitter.builder.build_int_add(lhs, rhs, "add"),
                            "*" => emitter.builder.build_int_mul(lhs, rhs, "mul"),
                            "==" => {
                                emit_compare_expression(emitter, "eq_int", lhs.into(), rhs.into())
                            }
                            ">" => {
                                emit_compare_expression(emitter, "sgt_int", lhs.into(), rhs.into())
                            }
                            "<" => {
                                emit_compare_expression(emitter, "slt_int", lhs.into(), rhs.into())
                            }
                            _ => panic!("unimpelemented operator."),
                        };
                        Value::Int(val)
                    }
                    Value::Float(rhs) => {
                        let lhs = lhs.const_signed_to_float(emitter.context.f32_type());
                        let val = match operator.as_ref() {
                            "+" => emitter.builder.build_float_add(lhs, rhs, "add"),
                            "*" => emitter.builder.build_float_mul(lhs, rhs, "mul"),
                            _ => panic!("unimpelemented operator."),
                        };
                        Value::Float(val)
                    }
                    _ => panic!("TODO"),
                },
                Value::Float(lhs) => match rhs {
                    Value::Int(rhs) => {
                        let rhs = rhs.const_signed_to_float(emitter.context.f32_type());
                        let val = match operator.as_ref() {
                            "+" => emitter.builder.build_float_add(lhs, rhs, "add"),
                            "*" => emitter.builder.build_float_mul(lhs, rhs, "mul"),
                            _ => panic!("unimpelemented operator."),
                        };
                        Value::Float(val)
                    }
                    Value::Float(rhs) => {
                        let val = match operator.as_ref() {
                            "+" => emitter.builder.build_float_add(lhs, rhs, "add"),
                            "*" => emitter.builder.build_float_mul(lhs, rhs, "mul"),
                            _ => panic!("unimpelemented operator."),
                        };
                        Value::Float(val)
                    }
                    _ => panic!("TODO"),
                },
                Value::Array(array_value, array_alloca, val_type, _size) => match rhs {
                    Value::Int(rhs) => match val_type {
                        BasicType::Int => {
                            let alloca = match operator.as_ref() {
                                "+" => unsafe {
                                    emitter.builder.build_gep(
                                        array_alloca,
                                        &[emitter.context.i32_type().const_int(0, false), rhs],
                                        "add",
                                    )
                                },
                                _ => panic!("unimpelemented operator."),
                            };
                            Value::Pointer(alloca, val_type)
                        }
                        _ => panic!("TODO"),
                    },
                    _ => panic!(),
                },
                _ => panic!("TODO"),
            }
        }
    }
}

fn emit_token(emitter: &mut Emitter, node: TokenNode) -> Value {
    match node.token {
        Token::IntNum(val) => {
            let val: u64 = val.parse().unwrap();
            let val = emitter.context.i32_type().const_int(val, false);
            Value::Int(val)
        }
        Token::FloatNum(val) => {
            let val: f64 = val.parse().unwrap();
            let val = emitter.context.f32_type().const_float(val);
            Value::Float(val)
        }
        Token::Ide(val) => {
            let identifier = val;
            match emitter.environment.get(&identifier) {
                Some((alloca, variable_type)) => match variable_type {
                    BasicType::Int => {
                        let val = emitter
                            .builder
                            .build_load(alloca, &identifier)
                            .into_int_value();
                        Value::Int(val)
                    }
                    BasicType::Float => {
                        let val = emitter
                            .builder
                            .build_load(alloca, &identifier)
                            .into_float_value();
                        Value::Float(val)
                    }
                    BasicType::Pointer(val_type) => {
                        let val = emitter
                            .builder
                            .build_load(alloca, &identifier)
                            .into_pointer_value();
                        Value::Pointer(val, *val_type)
                    }
                    BasicType::Array(val_type, size) => {
                        let val = emitter
                            .builder
                            .build_load(alloca, &identifier)
                            .into_array_value();
                        Value::Array(val, alloca, *val_type, size)
                    }
                    _ => panic!(),
                },
                None => panic!(format!("use of undeclared identifier {}", identifier)),
            }
        }
        _ => panic!(),
    }
}

fn emit_prefix(emitter: &mut Emitter, node: PrefixNode) -> Value {
    let prefix = node.prefix;
    let expression = *node.node;
    match prefix.token {
        Token::PrefixOp(op) => match op.as_ref() {
            "&" => {
                let val = emit_expression(emitter, expression);
                match val {
                    Value::Int(val) => {
                        let alloca = emitter
                            .builder
                            .build_alloca(emitter.context.i32_type(), "ref");
                        emitter.builder.build_store(alloca, val);
                        Value::Pointer(alloca, BasicType::Int)
                    }
                    _ => panic!(),
                }
            }
            "*" => {
                let val = emit_expression(emitter, expression);
                match val {
                    Value::Pointer(val, val_type) => match val_type {
                        BasicType::Int => {
                            Value::Int(emitter.builder.build_load(val, "star").into_int_value())
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            "++" => {
                let (alloca, alloca_type, identifier): (PointerValue, BasicType, String) =
                    emit_expression_as_pointer(emitter, expression);
                match alloca_type {
                    BasicType::Int => {
                        let val = emitter
                            .builder
                            .build_load(alloca, &identifier)
                            .into_int_value();
                        let const_one = emitter.context.i32_type().const_int(1, false);
                        let added_val = emitter.builder.build_int_add(val, const_one, "preinc");
                        emitter.builder.build_store(alloca, added_val);
                        Value::Int(added_val)
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn emit_suffix(emitter: &mut Emitter, node: SuffixNode) -> Value {
    let suffix = node.suffix;
    let expression = *node.node;
    let (alloca, val_type, identifier) = emit_expression_as_pointer(emitter, expression);
    match suffix.token {
        Token::SuffixOp(op) => match op.as_ref() {
            "++" => match val_type {
                BasicType::Int => {
                    let val = emitter
                        .builder
                        .build_load(alloca, &identifier)
                        .into_int_value();
                    let const_one = emitter.context.i32_type().const_int(1, false);
                    let incremented_val = emitter.builder.build_int_add(val, const_one, "postinc");
                    emitter.builder.build_store(alloca, incremented_val);
                    Value::Int(val)
                }
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn emit_array_index(emitter: &mut Emitter, node: ArrayIndexNode) -> Value {
    let (alloca, val_type, identifier) =
        emit_expression_as_pointer(emitter, ExpressionNode::ArrayIndex(node));

    let val = emitter.builder.build_load(alloca, &identifier);
    match val_type {
        BasicType::Int => Value::Int(val.into_int_value()),
        _ => panic!("TODO"),
    }
}

fn emit_function_call(emitter: &mut Emitter, node: FunctionCallNode) -> Value {
    let identifier = match node.identifier.token {
        Token::Ide(identifier) => identifier,
        _ => panic!(),
    };
    let fn_value = match emitter.module.get_function(&identifier) {
        Some(value) => value,
        None => panic!(format!("call of undeclared function {}", identifier)),
    };
    let vals = emit_comma_as_parameters(emitter, *node.parameters);

    let mut arguments: Vec<BasicValueEnum> = Vec::new();
    for val in vals.into_iter() {
        let argument: BasicValueEnum = match val {
            Value::Int(val) => val.into(),
            Value::Array(val, _alloca, _, _) => val.into(),
            _ => panic!("TODO"),
        };
        arguments.push(argument);
    }
    let func_call_site = emitter.builder.build_call(fn_value, &arguments, "func");
    let val = func_call_site
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();
    Value::Int(val)
}
