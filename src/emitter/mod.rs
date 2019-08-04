pub mod builtin;
pub mod const_expression;
pub mod environment;
pub mod expression;
pub mod statement;
pub mod util;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, FunctionType};

use std::path;

use crate::emitter::const_expression::*;
use crate::emitter::environment::*;
use crate::emitter::statement::*;
use crate::emitter::util::*;
use crate::parser::declare::*;
use crate::parser::statement::*;
use crate::parser::ProgramNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Control {
    Continue,
    Break,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NextBlock<'a> {
    continue_block: Option<&'a BasicBlock>,
    break_block: Option<&'a BasicBlock>,
}

pub struct Emitter {
    pub context: Context,
    pub builder: Builder,
    pub module: Module,
    pub env: Environment,
}
impl Emitter {
    pub fn new() -> Emitter {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("my_module");
        let env = Environment::new();
        Emitter {
            context,
            builder,
            module,
            env,
        }
    }
    pub fn print_to_file(&self) {
        let _ = self.module.print_to_file(path::Path::new("compiled.ll"));
    }
    pub fn emit(&mut self, node: ProgramNode) {
        emit_program(self, node);
    }
}

fn emit_program(emitter: &mut Emitter, node: ProgramNode) {
    // set builtin functions
    let i32_type = emitter.context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    emitter.module.add_function("eq_int", fn_type, None);
    emitter.module.add_function("sgt_int", fn_type, None);
    emitter.module.add_function("slt_int", fn_type, None);
    emitter.module.add_function("and_int", fn_type, None);
    emitter.module.add_function("or_int", fn_type, None);

    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    emitter.module.add_function("putchar", fn_type, None);

    let mut declares = node.declares;
    let scope = Scope::new(emitter);
    emitter.env.push_scope(scope);
    while let Some(declare) = declares.pop_front() {
        match declare {
            DeclareNode::Function(node) => emit_function(emitter, node),
            DeclareNode::Variable(node) => emit_declare_statement_global(emitter, node),
        };
    }
    emitter.env.pop_scope();
}

fn emit_function(emitter: &mut Emitter, function_node: FunctionNode) -> Control {
    let identifier = function_node.identifier;
    let mut statement_nodes = match function_node.statements {
        Some(statements) => statements, // with definition
        None => panic!("TODO"),         // only declare
    };
    let parameters = function_node.parameters;
    let mut param_types: Vec<BasicTypeEnum> = Vec::new();

    let mut cloned_paramaters = parameters.clone();
    while let Some(declare_variable_node) = cloned_paramaters.pop_front() {
        let val_type = declare_variable_node.value_type;
        let param_type = emitter.env.get_type_from_string(&val_type);
        param_types.push(param_type);
    }
    let scope = Scope::new(emitter);
    emitter.env.push_scope(scope);
    let return_type = emitter.env.get_type_from_string(&function_node.return_type);
    let fn_type = to_fn_type(return_type, param_types);
    let func = emitter.module.add_function(&identifier, fn_type, None);

    let basic_block = emitter.context.append_basic_block(&func, "entry");
    emitter.builder.position_at_end(&basic_block);

    for (i, declare_variable_node) in parameters.into_iter().enumerate() {
        // It can be used for assertion
        // let value_type = get_type_from_string(emitter, declare_variable_node.value_type);
        let identifier = declare_variable_node.identifier;
        let value = match func.get_nth_param(i as u32) {
            Some(val) => val,
            None => panic!("unexpected"),
        };
        let alloca = store_value(emitter, value);
        emitter
            .env
            .insert_new_other(identifier, Other::Variable(alloca));
    }

    let next_blocks = NextBlock {
        break_block: None,
        continue_block: None,
    };

    while let Some(statement_node) = statement_nodes.pop_front() {
        emit_statement(emitter, statement_node, next_blocks.clone());
    }
    Control::Continue
}

fn emit_declare_statement_global(emitter: &mut Emitter, node: DeclareStatementNode) -> Control {
    let node = node.declare_variable_node;

    let identifier = node.identifier;
    let value_type = emitter.env.get_type_from_string(&node.value_type.clone());

    let global = emitter.module.add_global(value_type, None, &identifier);
    if let Some(expression) = node.initialize_expression {
        let value = emit_const_expression(emitter, expression);
        global.set_initializer(&value)
    }
    emitter
        .env
        .insert_new_other(identifier, Other::Global(global));
    Control::Continue
}
