use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;

use std::path;

use crate::lexer::token::*;
use crate::parser::declare::DeclareNode;
use crate::parser::expression::node::{BinExpNode, ExpressionNode, TokenNode};
use crate::parser::statement::*;

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
    emit_statement(emitter, statement_node);
}

fn emit_statement(emitter: &mut Emitter, node: StatementNode) {
    let ret = emit_expression(emitter, node.expression);
    emitter.builder.build_return(Some(&ret));
}

fn emit_expression(emitter: &mut Emitter, node: ExpressionNode) -> IntValue {
    match node {
        ExpressionNode::BinExp(node) => emit_bin_exp(emitter, node),
        ExpressionNode::Token(node) => emit_token(emitter, node),
        _ => panic!(""),
    }
}

fn emit_bin_exp(emitter: &mut Emitter, node: BinExpNode) -> IntValue {
    let operator = match node.op.token {
        Token::Op(op, _) => op,
        _ => panic!(),
    };
    let lhs = emit_expression(emitter, *node.lhs);
    let rhs = emit_expression(emitter, *node.rhs);
    match operator.as_ref() {
        "+" => emitter.builder.build_int_add(lhs, rhs, "add"),
        "*" => emitter.builder.build_int_mul(lhs, rhs, "mul"),
        _ => panic!("unimpelemented operator."),
    }
}
fn emit_token(emitter: &mut Emitter, node: TokenNode) -> IntValue {
    match node.token {
        Token::Num(val) => emitter.context.i32_type().const_int(val as u64, false),
        _ => panic!(),
    }
}
