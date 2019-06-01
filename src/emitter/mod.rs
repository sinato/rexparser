use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;

use std::path;

use crate::lexer::token::*;
use crate::parser::declare::Node;
use crate::parser::expression::node::Node as ExpNode;
use crate::parser::expression::node::{BinExpNode, TokenNode};
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
    pub fn emit(&mut self, node: Node) {
        emit_function(self, node)
    }
}

fn emit_function(emitter: &mut Emitter, node: Node) {
    let function_node = match node {
        Node::Function(node) => node,
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
    let ret = match node.expression {
        ExpNode::BinExp(node) => emit_bin_exp(emitter, node),
        ExpNode::Token(node) => emit_token(emitter, node),
        _ => panic!(""),
    };
    emitter.builder.build_return(Some(&ret));
}

fn emit_expression(emitter: &mut Emitter, node: ExpNode) -> IntValue {
    match node {
        ExpNode::Token(node) => emit_token(emitter, node),
        _ => panic!(""),
    }
}

fn emit_bin_exp(emitter: &mut Emitter, node: BinExpNode) -> IntValue {
    let _operator = node.op;
    let lhs = emit_expression(emitter, *node.lhs);
    let rhs = emit_expression(emitter, *node.rhs);
    emitter.builder.build_int_add(lhs, rhs, "add")
}
fn emit_token(emitter: &mut Emitter, node: TokenNode) -> IntValue {
    match node.token {
        Token::Num(val) => emitter.context.i32_type().const_int(val as u64, false),
        _ => panic!(),
    }
}
