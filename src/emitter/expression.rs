use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, PointerValue};

use crate::emitter::util::*;
use crate::emitter::*;
use crate::lexer::token::*;
use crate::parser::expression::*;

pub fn emit_expression(emitter: &mut Emitter, node: ExpressionNode) -> PointerValue {
    match node {
        ExpressionNode::Token(node) => emit_token(emitter, node),
        ExpressionNode::BinExp(node) => emit_bin_exp(emitter, node),
        ExpressionNode::Prefix(node) => emit_prefix(emitter, node),
        ExpressionNode::ArrayIndex(node) => emit_array_index(emitter, node),
        ExpressionNode::FunctionCall(node) => emit_function_call(emitter, node),
        ExpressionNode::Suffix(node) => emit_suffix(emitter, node),
        ExpressionNode::Access(node) => emit_access(emitter, node),
        ExpressionNode::TernaryExp(node) => emit_ternary_exp(emitter, node),
        _ => panic!(format!("TODO {:?}", node)),
    }
}

fn emit_token(emitter: &mut Emitter, node: TokenNode) -> PointerValue {
    let default_int_type = emitter.context.i32_type();
    let default_float_type = emitter.context.f32_type();
    match node.token {
        Token::IntNum(val, _) => store_value(
            emitter,
            default_int_type.const_int_from_string(&val, 10).into(),
        ),
        Token::FloatNum(val, _) => store_value(
            emitter,
            default_float_type.const_float_from_string(&val).into(),
        ),
        Token::Ide(identifier, _) => match emitter.env.get_other(&identifier) {
            Some(other) => match other {
                Other::Variable(alloca) => alloca,
                Other::Global(alloca) => alloca.as_pointer_value(),
                _ => panic!(format!("TODO: {:?}", other)),
            },
            None => panic!(format!("{} is not exists", identifier)),
        },
        Token::Str(val, _) => {
            let s = unsafe { emitter.builder.build_global_string(&val, "str") };
            s.as_pointer_value()
        }
        _ => panic!("TODO"),
    }
}

fn emit_bin_exp(emitter: &mut Emitter, node: BinExpNode) -> PointerValue {
    let (operator, debug_info) = match node.op.token {
        Token::Op(op, debug_info) => (op, debug_info),
        _ => panic!("expects operator token"),
    };

    if is_assign_operator(&operator) {
        let alloca = emit_expression(emitter, *node.lhs);
        let value_alloca = emit_expression(emitter, *node.rhs);
        match operator.as_ref() {
            "=" => {
                let value = load_value(emitter, value_alloca);
                match alloca.get_type().get_element_type() {
                    AnyTypeEnum::IntType(int_type) => {
                        let value = cast_to_int(emitter, int_type, value);
                        emitter.builder.build_store(alloca, value);
                    }
                    AnyTypeEnum::FloatType(float_type) => {
                        let value = cast_to_float(emitter, float_type, value);
                        emitter.builder.build_store(alloca, value);
                    }
                    AnyTypeEnum::PointerType(pointer_type) => match value {
                        BasicValueEnum::PointerValue(pointer_value) => {
                            assert!(
                                pointer_type.get_element_type()
                                    == pointer_value.get_type().get_element_type()
                            );
                            emitter.builder.build_store(alloca, pointer_value);
                        }
                        _ => panic!(format!(
                            "expect pointer value, but {:?} at {:?}",
                            value, debug_info
                        )),
                    },
                    _ => panic!("TODO"),
                }
                alloca
            }
            "+=" => {
                let lhs_value = load_value(emitter, alloca);
                let rhs_value = load_value(emitter, value_alloca);

                match alloca.get_type().get_element_type() {
                    AnyTypeEnum::IntType(int_type) => {
                        let rhs_value = cast_to_int(emitter, int_type, rhs_value);
                        let added_value = emitter.builder.build_int_add(
                            lhs_value.into_int_value(),
                            rhs_value,
                            "plusequal",
                        );
                        emitter.builder.build_store(alloca, added_value);
                    }
                    _ => panic!("TODO"),
                }
                alloca
            }
            _ => panic!("TODO"),
        }
    } else {
        let lhs_alloca = emit_expression(emitter, *node.lhs);
        let rhs_alloca = emit_expression(emitter, *node.rhs);
        let lhs = load_value(emitter, lhs_alloca);
        let rhs = load_value(emitter, rhs_alloca);
        match lhs.get_type() {
            BasicTypeEnum::IntType(int_type) => {
                let lhs = lhs.into_int_value();
                let rhs = cast_to_int(emitter, int_type, rhs);
                match operator.as_ref() {
                    "+" => store_value(
                        emitter,
                        emitter.builder.build_int_add(lhs, rhs, "add").into(),
                    ),
                    "-" => store_value(
                        emitter,
                        emitter.builder.build_int_sub(lhs, rhs, "sub").into(),
                    ),
                    "*" => store_value(
                        emitter,
                        emitter.builder.build_int_mul(lhs, rhs, "mul").into(),
                    ),
                    "==" => {
                        let value = emit_compare_expression_int(emitter, "eq_int", lhs, rhs).into();
                        store_value(emitter, value)
                    }
                    ">" => {
                        let value =
                            emit_compare_expression_int(emitter, "sgt_int", lhs, rhs).into();
                        store_value(emitter, value)
                    }
                    "<" => {
                        let value =
                            emit_compare_expression_int(emitter, "slt_int", lhs, rhs).into();
                        store_value(emitter, value)
                    }
                    "&&" => {
                        let value =
                            emit_compare_expression_int(emitter, "and_int", lhs, rhs).into();
                        store_value(emitter, value)
                    }
                    "||" => {
                        let value = emit_compare_expression_int(emitter, "or_int", lhs, rhs).into();
                        store_value(emitter, value)
                    }
                    _ => panic!("TODO"),
                }
            }
            BasicTypeEnum::FloatType(float_type) => {
                let lhs = lhs.into_float_value();
                let rhs = cast_to_float(emitter, float_type, rhs);
                match operator.as_ref() {
                    "+" => store_value(
                        emitter,
                        emitter.builder.build_float_add(lhs, rhs, "add").into(),
                    ),
                    "*" => {
                        let value = emitter.builder.build_float_mul(lhs, rhs, "mul").into();
                        store_value(emitter, value)
                    }
                    _ => panic!("TODO"),
                }
            }
            BasicTypeEnum::ArrayType(_array_type) => {
                let rhs = if let BasicValueEnum::IntValue(value) = rhs {
                    value
                } else {
                    panic!("expects an integer value")
                };
                let const_zero = emitter.context.i32_type().const_zero();
                let value = unsafe {
                    emitter
                        .builder
                        .build_gep(lhs_alloca, &[const_zero, rhs], "gep")
                };
                store_value(emitter, BasicValueEnum::PointerValue(value))
            }
            _ => panic!(format!("TODO {:?}", lhs.get_type())),
        }
    }
}

fn emit_prefix(emitter: &mut Emitter, node: PrefixNode) -> PointerValue {
    let expression = *node.node;
    match node.prefix.token {
        Token::PrefixOp(op, _) => match op.as_ref() {
            "&" => {
                let value = BasicValueEnum::PointerValue(emit_expression(emitter, expression));
                store_value(emitter, value)
            }
            "*" => {
                let alloca = emit_expression(emitter, expression);
                let value_alloca = load_value(emitter, alloca);
                if let BasicValueEnum::PointerValue(value_alloca) = value_alloca {
                    let value = load_value(emitter, value_alloca);
                    store_value(emitter, value)
                } else {
                    panic!("")
                }
            }
            "++" => {
                let alloca = emit_expression(emitter, expression);
                let value = load_value(emitter, alloca);
                match value.get_type() {
                    BasicTypeEnum::IntType(_type) => {
                        let const_one = emitter.context.i32_type().const_int(1, false);
                        let incremented_val = emitter.builder.build_int_add(
                            value.into_int_value(),
                            const_one,
                            "postinc",
                        );
                        emitter.builder.build_store(alloca, incremented_val);
                        alloca
                    }
                    _ => panic!(),
                }
            }
            _ => panic!("TODO"),
        },
        _ => panic!("expects prefix operator"),
    }
}

fn emit_array_index(emitter: &mut Emitter, node: ArrayIndexNode) -> PointerValue {
    let array_alloca = emit_expression(emitter, *node.array);
    let index_alloca = emit_expression(emitter, *node.index);
    let index_value = load_value(emitter, index_alloca).into_int_value();
    let const_zero = emitter.context.i32_type().const_zero();
    match array_alloca.get_type().get_element_type() {
        AnyTypeEnum::PointerType(_type) => unsafe {
            let array_alloca = emitter
                .builder
                .build_load(array_alloca, "arr_pointer")
                .into_pointer_value();
            emitter
                .builder
                .build_gep(array_alloca, &[index_value], "arrindp")
        },
        AnyTypeEnum::ArrayType(_type) => unsafe {
            emitter
                .builder
                .build_gep(array_alloca, &[const_zero, index_value], "arrind")
        },
        _ => panic!("unexpected type"),
    }
}

fn emit_comma_as_arguments(emitter: &mut Emitter, node: ExpressionNode) -> Vec<BasicValueEnum> {
    let arguments = match node {
        ExpressionNode::BinExp(node) => match node.clone().op.token {
            Token::Op(op, _) => match op.as_ref() {
                "," => {
                    let mut lhs = emit_comma_as_arguments(emitter, *node.lhs);
                    let rhs = emit_comma_as_arguments(emitter, *node.rhs);
                    lhs.extend(rhs.iter().cloned());
                    lhs
                }
                _ => {
                    let val_ptr = emit_bin_exp(emitter, node);
                    vec![load_value(emitter, val_ptr)]
                }
            },
            _ => panic!(),
        },
        ExpressionNode::Token(node) => {
            let val_ptr = emit_token(emitter, node);
            vec![load_value(emitter, val_ptr)]
        }
        ExpressionNode::Empty => vec![],
        _ => panic!(),
    };
    let mut converted_arguments: Vec<BasicValueEnum> = Vec::new();
    for argument in arguments.into_iter() {
        let val = match argument.get_type() {
            BasicTypeEnum::ArrayType(_type) => {
                let const_zero = emitter.context.i32_type().const_zero();
                let array_alloca = store_value(emitter, argument);
                unsafe {
                    emitter
                        .builder
                        .build_gep(array_alloca, &[const_zero, const_zero], "arrptr")
                        .into()
                }
            }
            _ => argument,
        };
        converted_arguments.push(val)
    }
    converted_arguments
}

fn emit_function_call(emitter: &mut Emitter, node: FunctionCallNode) -> PointerValue {
    let identifier = match node.identifier.token {
        Token::Ide(identifier, _) => identifier,
        _ => panic!(),
    };
    let fn_value = match emitter.module.get_function(&identifier) {
        Some(value) => value,
        None => panic!(format!("call of undeclared function {}", identifier)),
    };
    let arguments = emit_comma_as_arguments(emitter, *node.parameters);
    let func_call_site = emitter.builder.build_call(fn_value, &arguments, "func");
    let val: BasicValueEnum = func_call_site.try_as_basic_value().left().unwrap();
    store_value(emitter, val)
}

fn emit_suffix(emitter: &mut Emitter, node: SuffixNode) -> PointerValue {
    let suffix = node.suffix;
    let expression = *node.node;
    let value_alloca = emit_expression(emitter, expression);
    let value = load_value(emitter, value_alloca);
    match suffix.token {
        Token::SuffixOp(op, _) => match op.as_ref() {
            "++" => match value.get_type() {
                BasicTypeEnum::IntType(_type) => {
                    let const_one = emitter.context.i32_type().const_int(1, false);
                    let incremented_val =
                        emitter
                            .builder
                            .build_int_add(value.into_int_value(), const_one, "postinc");
                    emitter.builder.build_store(value_alloca, incremented_val);
                    store_value(emitter, value) // return not incremented value
                }
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn emit_access(emitter: &mut Emitter, node: AccessNode) -> PointerValue {
    let access_identifier = if let Token::Ide(identifier, _) = node.access_identifier {
        identifier
    } else {
        panic!("unexpected")
    };
    let expression = *node.node;

    let identifier = if let ExpressionNode::Token(TokenNode {
        token: Token::Ide(identifier, _),
    }) = expression
    {
        identifier
    } else {
        panic!("unexpected")
    };
    let field_identifier = identifier + "." + &access_identifier;
    match emitter.env.get_other(&field_identifier) {
        Some(other) => match other {
            Other::Variable(value_alloca) => value_alloca,
            _ => panic!("unexpected"),
        },
        None => panic!("unexpected"),
    }
}

fn emit_ternary_exp(emitter: &mut Emitter, node: TernaryExpNode) -> PointerValue {
    let function = match emitter.module.get_last_function() {
        Some(func) => func,
        None => panic!(),
    };
    let entry_bb = function.get_last_basic_block().unwrap();
    let then_bb = function.append_basic_block("ifthen");
    let cont_bb = function.append_basic_block("ifcont");

    let then_val_alloca = emit_expression(emitter, *node.lhs);
    let entry_val_alloca = emit_expression(emitter, *node.rhs);

    let condition_val = emit_condition_expression_eq(emitter, *node.condition);
    emitter
        .builder
        .build_conditional_branch(condition_val, &cont_bb, &then_bb);

    emitter.builder.position_at_end(&then_bb);
    emitter.builder.build_unconditional_branch(&cont_bb);
    emitter.builder.position_at_end(&cont_bb);

    let phi_type = entry_val_alloca.get_type();
    let phi = emitter.builder.build_phi(phi_type, "compphi");

    phi.add_incoming(&[(&then_val_alloca, &then_bb), (&entry_val_alloca, &entry_bb)]);
    *phi.as_basic_value().as_pointer_value()
}
