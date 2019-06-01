use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FloatValue, IntValue};

use std::path;

use crate::lexer::token::*;
use crate::parser::declare::DeclareNode;
use crate::parser::expression::node::{BinExpNode, ExpressionNode, TokenNode};
use crate::parser::statement::*;

pub enum Value {
    Int(IntValue),
    Float(FloatValue),
}

pub struct Emitter {
    pub context: Context,
    pub builder: Builder,
    pub module: Module,
}
impl Emitter {
    pub fn new() -> Emitter {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("my_module");
        Emitter {
            context,
            builder,
            module,
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
    let statement_node = function_node.statement;

    let fn_type = emitter.context.i32_type().fn_type(&[], false);
    let func = emitter.module.add_function(&identifier, fn_type, None);

    let basic_block = emitter.context.append_basic_block(&func, "entry");
    emitter.builder.position_at_end(&basic_block);
    emit_statement(emitter, statement_node, function_node.return_type);
}

fn emit_statement(emitter: &mut Emitter, node: StatementNode, return_type: BasicType) {
    match node {
        StatementNode::Return(node) => emit_return_statement(emitter, node, return_type),
    }
}

fn emit_return_statement(emitter: &mut Emitter, node: ReturnStatementNode, return_type: BasicType) {
    let ret = emit_expression(emitter, node.expression);
    match return_type {
        BasicType::Int => {
            let ret = match ret {
                Value::Int(val) => val,
                Value::Float(val) => val.const_to_signed_int(emitter.context.i32_type()),
            };
            emitter.builder.build_return(Some(&ret));
        }
    }
}

fn emit_expression(emitter: &mut Emitter, node: ExpressionNode) -> Value {
    match node {
        ExpressionNode::BinExp(node) => emit_bin_exp(emitter, node),
        ExpressionNode::Token(node) => emit_token(emitter, node),
        _ => panic!(""),
    }
}

fn emit_bin_exp(emitter: &mut Emitter, node: BinExpNode) -> Value {
    let operator = match node.op.token {
        Token::Op(op, _) => op,
        _ => panic!(),
    };
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
        },
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
        _ => panic!(),
    }
}
