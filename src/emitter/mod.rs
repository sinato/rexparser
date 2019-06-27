pub mod builtin;
pub mod environment;
pub mod expression;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{ArrayValue, FloatValue, InstructionOpcode, IntValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};

use std::collections::VecDeque;
use std::path;

use crate::emitter::environment::Environment;
use crate::emitter::expression::util::*;
use crate::emitter::expression::*;
use crate::lexer::token::*;
use crate::parser::declare::*;
use crate::parser::statement::*;
use crate::parser::ProgramNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(IntValue),
    Float(FloatValue),
    Pointer(PointerValue, BasicType),
    Array(ArrayValue, PointerValue, BasicType, u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Control {
    Continue,
    Break,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NextBlocks<'a> {
    continue_block: Option<&'a BasicBlock>,
    break_block: Option<&'a BasicBlock>,
}

struct Delay<T, F> {
    value: Option<T>,
    func: F,
}
impl<T, F> Delay<T, F>
where
    F: Fn() -> T,
{
    fn new(f: F) -> Delay<T, F> {
        Delay {
            value: None,
            func: f,
        }
    }
    fn force(&mut self) -> &T {
        if self.value.is_none() {
            self.value = Some((self.func)());
        }
        self.value.as_ref().unwrap()
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

    emitter.environment.push_scope();
    let mut declares = node.declares;
    while let Some(declare) = declares.pop_front() {
        match declare {
            DeclareNode::Function(node) => emit_function(emitter, node),
            DeclareNode::Variable(node) => emit_declare_statement_global(emitter, node),
        };
    }
    emitter.environment.pop_scope();
}
fn emit_function(emitter: &mut Emitter, function_node: FunctionNode) -> Control {
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
    emitter.environment.push_scope();
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
            BasicType::Pointer(_) => {
                let value = basic_value.into_pointer_value();
                emitter.builder.build_store(alloca, value);
            }
            BasicType::Array(_, _) => {
                let value = basic_value.into_array_value();
                emitter.builder.build_store(alloca, value);
            }
            _ => panic!("TODO"),
        }
    }

    let next_blocks = NextBlocks {
        break_block: None,
        continue_block: None,
    };
    while let Some(statement_node) = statement_nodes.pop_front() {
        emit_statement(emitter, statement_node, next_blocks.clone());
    }
    emitter.environment.pop_scope();
    Control::Continue
}

fn emit_statement(emitter: &mut Emitter, node: StatementNode, next_block: NextBlocks) -> Control {
    match node {
        StatementNode::Expression(node) => emit_expression_statement(emitter, node),
        StatementNode::Return(node) => emit_return_statement(emitter, node),
        StatementNode::Declare(node) => emit_declare_statement(emitter, node),
        StatementNode::Struct(node) => emit_struct_statement(emitter, node),
        StatementNode::Enum(node) => emit_enum_statement(emitter, node),
        StatementNode::Compound(node) => emit_compound_statement(emitter, node, next_block),
        StatementNode::If(node) => emit_if_statement(emitter, node, next_block),
        StatementNode::While(node) => emit_while_statement(emitter, node),
        StatementNode::Break(node) => emit_break_statement(emitter, node, next_block),
        StatementNode::Continue(node) => emit_continue_statement(emitter, node, next_block),
        StatementNode::For(node) => emit_for_statement(emitter, node),
        StatementNode::Switch(node) => emit_switch_node(emitter, node),
        StatementNode::Case(_) | StatementNode::Default(_) => panic!("unexpected"),
    }
}

fn emit_compound_statement(
    emitter: &mut Emitter,
    node: CompoundStatementNode,
    next_block: NextBlocks,
) -> Control {
    let mut statements = node.statements;
    emitter.environment.push_scope();
    let mut control = Control::Continue;
    while let Some(statement) = statements.pop_front() {
        control = emit_statement(emitter, statement, next_block.clone());
        match control {
            Control::Continue => continue,
            Control::Break => break,
        }
    }
    emitter.environment.pop_scope();
    control
}

fn emit_expression_statement(emitter: &mut Emitter, node: ExpressionStatementNode) -> Control {
    emit_expression(emitter, node.expression);
    Control::Continue
}

fn emit_return_statement(emitter: &mut Emitter, node: ReturnStatementNode) -> Control {
    let ret = emit_expression(emitter, node.expression);
    let function = emitter.module.get_last_function().expect("a function");
    let return_type = function.get_return_type();
    match return_type {
        BasicTypeEnum::IntType(_) => {
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
    Control::Break
}

fn emit_declare_statement_global(emitter: &mut Emitter, node: DeclareStatementNode) -> Control {
    let node = node.declare_variable_node;
    let identifier = node.identifier;
    let value_type = node.value_type.clone();

    let value_type = get_nested_type(emitter, value_type);
    let global = emitter.module.add_global(value_type, None, &identifier);

    let initialize_exp = node.initialize_expression;
    if let Some(node) = initialize_exp {
        match emit_expression(emitter, node) {
            Value::Int(val) => global.set_initializer(&val),
            _ => panic!("TODO"),
        }
    }
    emitter
        .environment
        .insert_new(identifier, (global.as_pointer_value(), node.value_type));
    Control::Continue
}

fn emit_struct_statement(emitter: &mut Emitter, node: StructStatementNode) -> Control {
    match node {
        StructStatementNode::Definition(node) => {
            let struct_declare = node.target_struct;
            emitter
                .environment
                .insert_new_struct(struct_declare.clone().identifier, struct_declare);
        }
        StructStatementNode::Declare(node) => {
            let declare_variable_node = node.declare_variable_node;
            let identifier = declare_variable_node.identifier;
            let value_type = declare_variable_node.value_type;
            let struct_type = match value_type.clone() {
                BasicType::Struct(identifier) => {
                    let target_struct = emitter.environment.get_struct(&identifier).unwrap();
                    let mut members: Vec<BasicTypeEnum> = Vec::new();
                    for (_identifier, value_type) in target_struct.members {
                        let value_type = get_nested_type(emitter, value_type);
                        members.push(value_type);
                    }
                    emitter.context.struct_type(&members, false)
                }
                _ => panic!("unexpected pattern"),
            };
            let alloca = emitter.builder.build_alloca(struct_type, &identifier);
            emitter
                .environment
                .insert_new(identifier, (alloca, value_type));
        }
    }
    Control::Continue
}

fn emit_enum_statement(emitter: &mut Emitter, node: EnumStatementNode) -> Control {
    match node {
        EnumStatementNode::Definition(node) => {
            let _tag = node.tag;
            let enums = node.enums;

            for declare in enums {
                let alloca = emit_declare_statement_alloca(
                    emitter,
                    declare.identifier.clone(),
                    declare.value_type.clone(),
                );
                let initialize_exp = declare.initialize_expression;

                if let Some(node) = initialize_exp {
                    let val = emit_expression(emitter, node);
                    emit_equal_expression(emitter, alloca, declare.value_type.clone(), val);
                }
                emitter
                    .environment
                    .insert_new(declare.identifier, (alloca, declare.value_type));
            }

            Control::Continue
        }
        EnumStatementNode::Declare(node) => emit_declare_statement(emitter, node),
    }
}

fn emit_declare_statement(emitter: &mut Emitter, node: DeclareStatementNode) -> Control {
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
    Control::Continue
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

fn emit_if_statement(
    emitter: &mut Emitter,
    node: IfStatementNode,
    next_blocks: NextBlocks,
) -> Control {
    // ---- condition ---- ifthen ---- ifcont
    //          ┗------------------------┛
    //
    let function = match emitter.module.get_last_function() {
        Some(func) => func,
        None => panic!(),
    };
    match node.else_block {
        Some(else_block) => {
            let then_bb = function.append_basic_block("ifthen");
            let else_bb = function.append_basic_block("ifelse");
            let mut lazy_cont_bb = Delay::new(|| function.append_basic_block("ifcont"));

            let condition_val = match emit_expression(emitter, node.condition_expression) {
                Value::Int(val) => val,
                _ => panic!("TODO"),
            };
            let const_one = emitter.context.i32_type().const_int(0, false);
            let condition_val =
                emitter
                    .builder
                    .build_int_compare(IntPredicate::EQ, condition_val, const_one, "eq");
            emitter
                .builder
                .build_conditional_branch(condition_val, &else_bb, &then_bb);

            emitter.builder.position_at_end(&then_bb);
            let control_if = emit_statement(emitter, *node.block, next_blocks.clone());
            if control_if == Control::Continue {
                let cont_bb = lazy_cont_bb.force();
                emitter.builder.build_unconditional_branch(&cont_bb);
            }

            emitter.builder.position_at_end(&else_bb);
            let control_else = emit_statement(emitter, *else_block, next_blocks);
            if control_else == Control::Continue {
                let cont_bb = lazy_cont_bb.force();
                emitter.builder.build_unconditional_branch(&cont_bb);
            }

            if control_if != Control::Break || control_else != Control::Break {
                let cont_bb = lazy_cont_bb.force();
                emitter.builder.position_at_end(&cont_bb);
                Control::Continue
            } else {
                Control::Break
            }
        }
        None => {
            let then_bb = function.append_basic_block("ifthen");
            let cont_bb = function.append_basic_block("ifcont");

            let condition_val = match emit_expression(emitter, node.condition_expression) {
                Value::Int(val) => val,
                _ => panic!("TODO"),
            };
            let const_one = emitter.context.i32_type().const_int(0, false);
            let condition_val =
                emitter
                    .builder
                    .build_int_compare(IntPredicate::EQ, condition_val, const_one, "eq");

            emitter
                .builder
                .build_conditional_branch(condition_val, &cont_bb, &then_bb);

            emitter.builder.position_at_end(&then_bb);
            emit_statement(emitter, *node.block, next_blocks);
            emitter.builder.build_unconditional_branch(&cont_bb);

            emitter.builder.position_at_end(&cont_bb);
            Control::Continue
        }
    }
}

fn emit_while_statement(emitter: &mut Emitter, node: WhileStatementNode) -> Control {
    // ---- comp ---- then ---- cont
    //       ┗--------------------┛
    let function = match emitter.module.get_last_function() {
        Some(func) => func,
        None => panic!(),
    };
    let comp_bb = function.append_basic_block("comp");
    let then_bb = function.append_basic_block("then");
    let cont_bb = function.append_basic_block("cont");

    emitter.builder.build_unconditional_branch(&comp_bb);

    emitter.builder.position_at_end(&comp_bb);
    let condition_val = match emit_expression(emitter, node.condition_expression) {
        Value::Int(val) => val,
        _ => panic!("TODO"),
    };
    let const_one = emitter.context.i32_type().const_int(0, false);
    let condition_val =
        emitter
            .builder
            .build_int_compare(IntPredicate::EQ, condition_val, const_one, "eq");
    emitter
        .builder
        .build_conditional_branch(condition_val, &cont_bb, &then_bb);

    emitter.builder.position_at_end(&then_bb);
    let next_blocks = NextBlocks {
        break_block: Some(&cont_bb),
        continue_block: Some(&comp_bb),
    };
    emit_statement(emitter, *node.block, next_blocks);
    emitter.builder.build_unconditional_branch(&comp_bb);

    emitter.builder.position_at_end(&cont_bb);
    Control::Continue
}

fn emit_continue_statement(
    emitter: &mut Emitter,
    _node: ContinueStatementNode,
    next_block: NextBlocks,
) -> Control {
    match next_block.continue_block {
        Some(next_block) => emitter.builder.build_unconditional_branch(next_block),
        None => panic!(""),
    };
    Control::Break
}

fn emit_break_statement(
    emitter: &mut Emitter,
    _node: BreakStatementNode,
    next_block: NextBlocks,
) -> Control {
    match next_block.break_block {
        Some(next_block) => emitter.builder.build_unconditional_branch(next_block),
        None => panic!(""),
    };
    Control::Break
}

fn emit_for_statement(emitter: &mut Emitter, node: ForStatementNode) -> Control {
    // setup
    let function = match emitter.module.get_last_function() {
        Some(func) => func,
        None => panic!(),
    };
    let comp_bb = function.append_basic_block("comp");
    let then_bb = function.append_basic_block("then");
    let thir_bb = function.append_basic_block("thir");
    let cont_bb = function.append_basic_block("cont");

    // emit first statement
    emitter.environment.push_scope();
    let next_blocks = NextBlocks {
        break_block: None,
        continue_block: None,
    };
    emit_statement(emitter, *node.first_statement, next_blocks);
    emitter.builder.build_unconditional_branch(&comp_bb);

    // check condition
    emitter.builder.position_at_end(&comp_bb);
    let condition_val = match emit_expression(emitter, node.condition_expression) {
        Value::Int(val) => val,
        _ => panic!("TODO"),
    };
    let const_one = emitter.context.i32_type().const_int(0, false);
    let condition_val =
        emitter
            .builder
            .build_int_compare(IntPredicate::EQ, condition_val, const_one, "foreq");
    emitter
        .builder
        .build_conditional_branch(condition_val, &cont_bb, &then_bb);

    // emit the block
    emitter.builder.position_at_end(&then_bb);
    let next_blocks = NextBlocks {
        break_block: Some(&cont_bb),
        continue_block: Some(&thir_bb),
    };
    emit_statement(emitter, *node.block, next_blocks);
    emitter.builder.build_unconditional_branch(&thir_bb);

    emitter.builder.position_at_end(&thir_bb);
    emit_expression(emitter, node.loop_expression);
    emitter.builder.build_unconditional_branch(&comp_bb);

    emitter.builder.position_at_end(&cont_bb);

    emitter.environment.pop_scope();
    Control::Continue
}

fn emit_switch_node(emitter: &mut Emitter, node: SwitchStatementNode) -> Control {
    let condition_expression = node.condition_expression;
    let statements = node.statements.statements;

    let function = match emitter.module.get_last_function() {
        Some(func) => func,
        None => panic!(),
    };

    // create basic blocks
    let mut cmp_bbs: VecDeque<BasicBlock> = VecDeque::new();
    let mut case_bbs: VecDeque<BasicBlock> = VecDeque::new();

    let mut statements_for_bb = statements.clone();
    let mut need_cmp_bb = true;
    let entry_bb = function.append_basic_block("entry");
    while let Some(statement) = statements_for_bb.pop_front() {
        match statement {
            StatementNode::Case(_) => {
                let case_bb = function.append_basic_block("case");
                if need_cmp_bb {
                    let cmp_bb = function.append_basic_block("cmp");
                    cmp_bbs.push_back(cmp_bb);
                }
                case_bbs.push_back(case_bb);
            }
            StatementNode::Default(_) => {
                let default_bb = function.append_basic_block("default");
                case_bbs.push_back(default_bb);

                let cmp_bb = function.append_basic_block("cmp");
                cmp_bbs.push_back(cmp_bb);
                need_cmp_bb = false;
            }
            _ => panic!("TODO"),
        }
    }
    let cont_bb = function.append_basic_block("cont");
    let nb = NextBlocks {
        continue_block: None,
        break_block: Some(&cont_bb),
    };

    // Scenario 1: Default statement exists.
    // entry --> cmp1 ---> cmp2 ---> cmp3        cont
    //            |         |         |           |
    //           case1 --> case2 --> default --> case3
    //
    // cmp_bbs: [cmp1, cmp2, cmp3]
    // case_bbs: [case1, case2, default, case3]
    //
    // Scenario 2: Default statement does not exist.
    // entry --> cmp1 ---> cmp2 ---> cmp3 --> cont
    //            |         |         |         |
    //           case1 --> case2 --> case3 -----|
    //
    // cmp_bbs: [cmp1, cmp2, cmp3]
    // case_bbs: [case1, case2, case3]
    //
    // Scenario 3: Any case / default statement does not exist.
    // entry --> cont
    //
    // cmp_bbs: []
    // case_bbs: []

    // entry ==============================================================
    emitter.builder.build_unconditional_branch(&entry_bb);
    emitter.builder.position_at_end(&entry_bb);

    let first_cmp_bb_ref = if cmp_bbs.len() > 0 {
        &cmp_bbs[0]
    } else {
        &cont_bb
    };
    let condition_val = match emit_expression(emitter, condition_expression) {
        Value::Int(val) => val,
        _ => panic!("unexpcted(switch statement expects an integer value.)"),
    };
    emitter.builder.build_unconditional_branch(first_cmp_bb_ref);

    // cmp
    let mut statements_for_cmp = statements.clone();
    let cmp_bbs_len = cmp_bbs.len();
    for idx in 0..cmp_bbs.len() {
        let cmp_bb_ref = &cmp_bbs[idx];
        let next_cmp_bb_ref = if idx + 1 < cmp_bbs_len {
            &cmp_bbs[idx + 1]
        } else {
            &cont_bb
        };
        let case_bb_ref = &case_bbs[idx];

        emitter.builder.position_at_end(cmp_bb_ref);

        match statements_for_cmp.pop_front() {
            Some(statement) => match statement {
                StatementNode::Case(statement) => {
                    let case_condition_val =
                        match emit_expression(emitter, statement.condition_expression) {
                            Value::Int(val) => val,
                            _ => panic!("unexpcted(switch statement expects an integer value.)"),
                        };
                    let cmp_val = emitter.builder.build_int_compare(
                        IntPredicate::EQ,
                        case_condition_val,
                        condition_val,
                        "caseeq",
                    );
                    emitter
                        .builder
                        .build_conditional_branch(cmp_val, case_bb_ref, next_cmp_bb_ref);
                }
                StatementNode::Default(_) => {
                    emitter.builder.build_unconditional_branch(case_bb_ref);
                }
                _ => panic!("TODO"),
            },
            None => panic!("unexpected"),
        }
    }

    // case
    let statements_size = statements.len();
    for (idx, statement) in statements.into_iter().enumerate() {
        emitter.builder.position_at_end(&case_bbs[idx]);
        let block_statements = match statement {
            StatementNode::Case(statement) => statement.statements,
            StatementNode::Default(statement) => statement.statements,
            _ => panic!(),
        };
        let next_bb_ref = match emit_compound_statement(emitter, block_statements, nb.clone()) {
            Control::Continue => {
                if idx + 1 < statements_size {
                    &case_bbs[idx + 1]
                } else {
                    &cont_bb
                }
            }
            Control::Break => &cont_bb,
        };
        emitter.builder.build_unconditional_branch(next_bb_ref);
    }

    emitter.builder.position_at_end(&cont_bb);
    Control::Continue
}
