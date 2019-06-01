use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

use std::path;

use crate::parser::declare::Node;
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
    let ret = emitter.context.i32_type().const_int(node.val as u64, false);
    emitter.builder.build_return(Some(&ret));
}
