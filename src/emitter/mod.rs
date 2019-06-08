pub mod environment;
pub mod expression;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{ArrayValue, FloatValue, InstructionOpcode, IntValue, PointerValue};
use inkwell::AddressSpace;

use std::path;

use crate::emitter::environment::Environment;
use crate::emitter::expression::util::*;
use crate::emitter::expression::*;
use crate::lexer::token::*;
use crate::parser::declare::DeclareNode;
use crate::parser::statement::*;
use crate::parser::ProgramNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(IntValue),
    Float(FloatValue),
    Pointer(PointerValue, BasicType),
    Array(ArrayValue, PointerValue, BasicType, u32),
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
    pub fn emit(&mut self, node: ProgramNode) {
        emit_program(self, node);
    }
}

fn emit_program(emitter: &mut Emitter, node: ProgramNode) {
    let mut declares = node.declares;
    while let Some(declare) = declares.pop_front() {
        emit_function(emitter, declare)
    }
}
fn emit_function(emitter: &mut Emitter, node: DeclareNode) {
    let function_node = match node {
        DeclareNode::Function(node) => node,
    };
    let identifier = function_node.identifier;
    let mut statement_nodes = function_node.statements;

    let parameters = function_node.parameters;
    let mut param_types: Vec<BasicTypeEnum> = Vec::new();

    let mut cparameters = parameters.clone();

    while let Some(declare_variable_node) = cparameters.pop_front() {
        let val_type = declare_variable_node.value_type;
        let param_type = get_nested_type(emitter, val_type);
        param_types.push(param_type);
    }
    let fn_type = emitter.context.i32_type().fn_type(&param_types, false);
    let func = emitter.module.add_function(&identifier, fn_type, None);

    let basic_block = emitter.context.append_basic_block(&func, "entry");
    emitter.builder.position_at_end(&basic_block);

    for (i, declare_variable_node) in parameters.into_iter().enumerate() {
        let value_type = declare_variable_node.value_type;
        let identifier = declare_variable_node.identifier;

        let inkwell_value_type = get_nested_type(emitter, value_type.clone());
        let alloca = emitter
            .builder
            .build_alloca(inkwell_value_type, &identifier);
        emitter
            .environment
            .insert_new(identifier, (alloca, value_type.clone()));

        let basic_value = match func.get_nth_param(i as u32) {
            Some(val) => val,
            None => panic!(),
        };
        match value_type {
            BasicType::Int => {
                let value = basic_value.into_int_value();
                emitter.builder.build_store(alloca, value);
            }
            BasicType::Array(_, _) => {
                let value = basic_value.into_array_value();
                emitter.builder.build_store(alloca, value);
            }
            _ => panic!("TODO"),
        }
    }

    while let Some(statement_node) = statement_nodes.pop_front() {
        emit_statement(emitter, statement_node, function_node.return_type.clone());
    }
}

fn emit_statement(emitter: &mut Emitter, node: StatementNode, return_type: BasicType) {
    match node {
        StatementNode::Expression(node) => emit_expression_statement(emitter, node),
        StatementNode::Return(node) => emit_return_statement(emitter, node, return_type),
        StatementNode::Declare(node) => emit_declare_statement(emitter, node),
        StatementNode::Compound(node) => emit_compound_statement(emitter, node),
    }
}

fn emit_compound_statement(emitter: &mut Emitter, node: CompoundStatementNode) {
    let mut statements = node.statements;
    emitter.environment.push_scope();
    while let Some(statement) = statements.pop_front() {
        emit_statement(emitter, statement, BasicType::Empty);
    }
    emitter.environment.pop_scope();
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
    let node = node.declare_variable_node;
    let identifier = node.identifier;
    let value_type = node.value_type;
    let alloca = emit_declare_statement_alloca(emitter, identifier.clone(), value_type.clone());

    let initialize_exp = node.initialize_expression;
    if let Some(node) = initialize_exp {
        let val = emit_expression(emitter, node);
        emit_equal_expression(emitter, alloca, value_type.clone(), val);
    }
    emitter
        .environment
        .insert_new(identifier, (alloca, value_type));
}

fn emit_declare_statement_alloca(
    emitter: &mut Emitter,
    identifier: String,
    value_type: BasicType,
) -> PointerValue {
    let value_type = get_nested_type(emitter, value_type);
    emitter.builder.build_alloca(value_type, &identifier)
}

fn get_nested_type(emitter: &mut Emitter, value_type: BasicType) -> BasicTypeEnum {
    match value_type {
        BasicType::Int => BasicTypeEnum::IntType(emitter.context.i32_type()),
        BasicType::Float => BasicTypeEnum::FloatType(emitter.context.f32_type()),
        BasicType::Pointer(boxed_type) => {
            // I do not know anything about the address space.
            let pointer_type = match get_nested_type(emitter, *boxed_type) {
                BasicTypeEnum::IntType(value_type) => value_type.ptr_type(AddressSpace::Generic),
                BasicTypeEnum::FloatType(value_type) => value_type.ptr_type(AddressSpace::Generic),
                BasicTypeEnum::PointerType(value_type) => {
                    value_type.ptr_type(AddressSpace::Generic)
                }
                BasicTypeEnum::ArrayType(value_type) => value_type.ptr_type(AddressSpace::Generic),
                _ => panic!(),
            };
            BasicTypeEnum::PointerType(pointer_type)
        }
        BasicType::Array(boxed_type, size) => {
            let array_type = match get_nested_type(emitter, *boxed_type) {
                BasicTypeEnum::IntType(value_type) => value_type.array_type(size),
                BasicTypeEnum::FloatType(value_type) => value_type.array_type(size),
                BasicTypeEnum::PointerType(value_type) => value_type.array_type(size),
                BasicTypeEnum::ArrayType(value_type) => value_type.array_type(size),
                _ => panic!(),
            };
            BasicTypeEnum::ArrayType(array_type)
        }
        _ => panic!(),
    }
}
