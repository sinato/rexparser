use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::InstructionOpcode;
use inkwell::values::{FloatValue, IntValue, PointerValue};

use std::collections::HashMap;
use std::path;

use crate::lexer::token::*;
use crate::parser::declare::DeclareNode;
use crate::parser::expression::node::*;
use crate::parser::statement::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(IntValue),
    Float(FloatValue),
    Pointer(PointerValue, BasicType),
}

pub struct Environment {
    variables: HashMap<String, (PointerValue, BasicType)>,
}
impl Environment {
    fn new() -> Environment {
        let variables: HashMap<String, (PointerValue, BasicType)> = HashMap::new();
        Environment { variables }
    }
    fn insert(
        &mut self,
        key: String,
        value: (PointerValue, BasicType),
    ) -> Option<(PointerValue, BasicType)> {
        self.variables.insert(key, value)
    }
}

pub struct Emitter {
    pub context: Context,
    pub builder: Builder,
    pub module: Module,
    pub environment: Environment,
}
impl Emitter {
    pub fn new() -> Emitter {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("my_module");
        let environment = Environment::new();
        Emitter {
            context,
            builder,
            module,
            environment,
        }
    }
    pub fn print_to_file(&self) {
        let _ = self.module.print_to_file(path::Path::new("compiled.ll"));
    }
    pub fn emit(&mut self, node: DeclareNode) {
        emit_function(self, node)
    }
}

fn emit_function(emitter: &mut Emitter, node: DeclareNode) {
    let function_node = match node {
        DeclareNode::Function(node) => node,
    };
    let identifier = function_node.identifier;
    let mut statement_nodes = function_node.statements;

    let fn_type = emitter.context.i32_type().fn_type(&[], false);
    let func = emitter.module.add_function(&identifier, fn_type, None);

    let basic_block = emitter.context.append_basic_block(&func, "entry");
    emitter.builder.position_at_end(&basic_block);
    while let Some(statement_node) = statement_nodes.pop_front() {
        emit_statement(emitter, statement_node, function_node.return_type.clone());
    }
}

fn emit_statement(emitter: &mut Emitter, node: StatementNode, return_type: BasicType) {
    match node {
        StatementNode::Expression(node) => emit_expression_statement(emitter, node),
        StatementNode::Return(node) => emit_return_statement(emitter, node, return_type),
        StatementNode::Declare(node) => emit_declare_statement(emitter, node),
    }
}

fn emit_expression_statement(emitter: &mut Emitter, node: ExpressionStatementNode) {
    emit_expression(emitter, node.expression);
}

fn emit_return_statement(emitter: &mut Emitter, node: ReturnStatementNode, return_type: BasicType) {
    let ret = emit_expression(emitter, node.expression);
    match return_type {
        BasicType::Int => {
            let ret = match ret {
                Value::Int(val) => val,
                Value::Float(val) => {
                    let opcode = InstructionOpcode::FPToSI;
                    let to_type = emitter.context.i32_type();
                    emitter
                        .builder
                        .build_cast(opcode, val, to_type, "convert")
                        .into_int_value()
                }
                _ => panic!("TODO"),
            };
            emitter.builder.build_return(Some(&ret));
        }
        _ => panic!(),
    }
}

fn emit_declare_statement(emitter: &mut Emitter, node: DeclareStatementNode) {
    let identifier = node.identifier;
    let value_type = node.value_type;
    let alloca = emit_declare_statement_alloca(emitter, identifier.clone(), value_type.clone());
    emitter.environment.insert(identifier, (alloca, value_type));
}
fn emit_declare_statement_alloca(
    emitter: &mut Emitter,
    identifier: String,
    value_type: BasicType,
) -> PointerValue {
    match value_type {
        BasicType::Int => emitter
            .builder
            .build_alloca(emitter.context.i32_type(), &identifier),
        BasicType::Float => emitter
            .builder
            .build_alloca(emitter.context.f32_type(), &identifier),
        BasicType::Pointer(boxed_type) => {
            emit_declare_statement_alloca(emitter, identifier, *boxed_type)
        }
        BasicType::Array(boxed_type, size) => {
            let array_type = match *boxed_type {
                BasicType::Int => emitter.context.i32_type().array_type(size),
                _ => panic!("TODO"),
            };
            emitter.builder.build_alloca(array_type, "")
        }
    }
}

fn emit_expression(emitter: &mut Emitter, node: ExpressionNode) -> Value {
    match node {
        ExpressionNode::BinExp(node) => emit_bin_exp(emitter, node),
        ExpressionNode::Token(node) => emit_token(emitter, node),
        ExpressionNode::Prefix(node) => emit_prefix(emitter, node),
        ExpressionNode::ArrayIndex(node) => emit_array_index(emitter, node),
        _ => panic!(""),
    }
}

fn emit_expression_as_pointer(
    emitter: &mut Emitter,
    node: ExpressionNode,
) -> (PointerValue, BasicType, String) {
    match node {
        ExpressionNode::BinExp(_node) => panic!("TODO: implement"),
        ExpressionNode::Token(node) => match node.token {
            Token::Ide(identifier) => {
                match emitter.environment.variables.clone().remove(&identifier) {
                    Some((alloca, variable_type)) => (alloca, variable_type, identifier),
                    None => panic!(format!("use of undeclared identifier {}", identifier)),
                }
            }
            _ => panic!(),
        },
        ExpressionNode::ArrayIndex(node) => {
            let array: ExpressionNode = *node.array;
            let index: ExpressionNode = *node.index;

            let index = match emit_expression(emitter, index) {
                Value::Int(val) => val,
                _ => panic!(),
            };

            // get idenfitier from arrayNode
            let identifier = match array.clone() {
                ExpressionNode::Token(token_node) => match token_node.token {
                    Token::Ide(identifier) => identifier,
                    _ => panic!(),
                },
                _ => panic!(),
            };
            let (array_alloca, val_type) = match emit_expression(emitter, array) {
                Value::Pointer(alloca, val_type) => (alloca, val_type),
                _ => panic!(),
            };

            match val_type {
                BasicType::Int => {
                    let alloca = unsafe {
                        emitter.builder.build_gep(
                            array_alloca,
                            &[emitter.context.i32_type().const_int(0, false), index],
                            "",
                        )
                    };
                    (alloca, BasicType::Int, identifier)
                }
                _ => panic!("TODO"),
            }
        }
        _ => panic!(),
    }
}

fn emit_prefix(emitter: &mut Emitter, node: PrefixNode) -> Value {
    let prefix = node.prefix;
    let expression = *node.node;
    let val = emit_expression(emitter, expression);
    match prefix.token {
        Token::PrefixOp(op) => match op.as_ref() {
            "&" => match val {
                Value::Int(val) => {
                    let alloca = emitter.builder.build_alloca(emitter.context.i32_type(), "");
                    emitter.builder.build_store(alloca, val);
                    Value::Pointer(alloca, BasicType::Int)
                }
                _ => panic!(),
            },
            "*" => match val {
                Value::Pointer(val, val_type) => match val_type {
                    BasicType::Int => {
                        Value::Int(emitter.builder.build_load(val, "").into_int_value())
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn emit_bin_exp(emitter: &mut Emitter, node: BinExpNode) -> Value {
    let operator = match node.op.token {
        Token::Op(op, _) => op,
        _ => panic!(),
    };
    match operator.as_ref() {
        "=" => {
            let (alloca, variable_type, identifier): (PointerValue, BasicType, String) =
                emit_expression_as_pointer(emitter, *node.lhs);
            let val: Value = emit_expression(emitter, *node.rhs);
            match variable_type {
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
                                emitter
                                    .environment
                                    .insert(identifier, (val, BasicType::Pointer(boxed_type)));
                                Value::Pointer(val, val_type)
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
        _ => {
            let lhs = emit_expression(emitter, *node.lhs);
            let rhs = emit_expression(emitter, *node.rhs);

            match lhs {
                Value::Int(lhs) => match rhs {
                    Value::Int(rhs) => {
                        let val = match operator.as_ref() {
                            "+" => emitter.builder.build_int_add(lhs, rhs, "add"),
                            "*" => emitter.builder.build_int_mul(lhs, rhs, "mul"),
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
            match emitter.environment.variables.clone().remove(&identifier) {
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
                    BasicType::Pointer(val_type) => Value::Pointer(alloca, *val_type),
                    BasicType::Array(val_type, _size) => Value::Pointer(alloca, *val_type),
                },
                None => panic!(format!("use of undeclared identifier {}", identifier)),
            }
        }
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
