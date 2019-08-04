use inkwell::types::BasicTypeEnum;
use inkwell::values::PointerValue;
use inkwell::IntPredicate;

use crate::emitter::environment::*;
use crate::emitter::expression::*;
use crate::emitter::util::*;
use crate::emitter::*;
use crate::parser::statement::*;

use std::collections::VecDeque;

pub fn emit_statement(
    emitter: &mut Emitter,
    node: StatementNode,
    next_block: NextBlock,
) -> Control {
    match node {
        StatementNode::Return(node) => emit_return_statement(emitter, node),
        StatementNode::Declare(node) => emit_declare_statement(emitter, node),
        StatementNode::Expression(node) => emit_expression_statement(emitter, node),
        StatementNode::Compound(node) => emit_compound_statement(emitter, node, next_block),
        StatementNode::If(node) => emit_if_statement(emitter, node, next_block),
        StatementNode::While(node) => emit_while_statement(emitter, node),
        StatementNode::Switch(node) => emit_switch_statement(emitter, node),
        StatementNode::Continue(node) => emit_continue_statement(emitter, node, next_block),
        StatementNode::Break(node) => emit_break_statement(emitter, node, next_block),
        StatementNode::For(node) => emit_for_statement(emitter, node),
        StatementNode::Struct(node) => emit_struct_statement(emitter, node),
        StatementNode::Enum(node) => emit_enum_statement(emitter, node),
        StatementNode::Empty => Control::Continue,
        _ => panic!(format!("TODO: {:?}", node)),
    }
}

fn emit_return_statement(emitter: &mut Emitter, node: ReturnStatementNode) -> Control {
    let return_value = emit_expression(emitter, node.expression);
    let function = emitter.module.get_last_function().expect("a function");
    let return_type = function.get_return_type();
    match return_type {
        BasicTypeEnum::IntType(int_type) => {
            let ret = load_value(emitter, return_value);
            let ret = cast_to_int(emitter, int_type, ret);
            emitter.builder.build_return(Some(&ret));
        }
        _ => panic!("TODO"),
    }
    Control::Break
}

pub fn alloca_from_basic_type(emitter: &mut Emitter, basic_type: BasicTypeEnum) -> PointerValue {
    let alloca = match basic_type {
        BasicTypeEnum::IntType(int_type) => emitter.builder.build_alloca(int_type, "alloca_int"),
        BasicTypeEnum::FloatType(float_type) => {
            emitter.builder.build_alloca(float_type, "alloca_float")
        }
        BasicTypeEnum::PointerType(pointer_type) => {
            emitter.builder.build_alloca(pointer_type, "alloca_ptr")
        }
        BasicTypeEnum::ArrayType(array_type) => {
            emitter.builder.build_alloca(array_type, "alloca_arr")
        }
        BasicTypeEnum::StructType(struct_type) => {
            emitter.builder.build_alloca(struct_type, "alloca_struct")
        }
        _ => panic!(format!("TODO {:?}", basic_type)),
    };
    alloca
}

fn emit_declare_statement(emitter: &mut Emitter, node: DeclareStatementNode) -> Control {
    let node = node.declare_variable_node;

    let identifier = node.identifier;
    let value_type = emitter.env.get_type_from_string(&node.value_type.clone());

    let alloca = if let Some(node) = node.initialize_expression {
        emit_expression(emitter, node)
    } else {
        alloca_from_basic_type(emitter, value_type)
    };
    emitter
        .env
        .insert_new_other(identifier, Other::Variable(alloca));
    Control::Continue
}

fn emit_expression_statement(emitter: &mut Emitter, node: ExpressionStatementNode) -> Control {
    emit_expression(emitter, node.expression);
    Control::Continue
}

fn emit_compound_statement(
    emitter: &mut Emitter,
    node: CompoundStatementNode,
    next_block: NextBlock,
) -> Control {
    let mut statements = node.statements;
    let scope = Scope::new(emitter);
    emitter.env.push_scope(scope);
    let mut control = Control::Continue;
    while let Some(statement) = statements.pop_front() {
        control = emit_statement(emitter, statement, next_block.clone());
        match control {
            Control::Continue => continue,
            Control::Break => break,
        }
    }
    emitter.env.pop_scope();
    control
}

fn emit_if_statement(
    emitter: &mut Emitter,
    node: IfStatementNode,
    next_block: NextBlock,
) -> Control {
    // ---- condition ---- ifthen ---- ifcont
    //          ┗------------------------┛
    //
    let function = match emitter.module.get_last_function() {
        Some(func) => func,
        None => panic!(),
    };
    let condition_val = emit_condition_expression_eq(emitter, node.condition_expression);
    match node.else_block {
        Some(else_block) => {
            let then_bb = function.append_basic_block("ifthen");
            let else_bb = function.append_basic_block("ifelse");
            let mut lazy_cont_bb = Delay::new(|| function.append_basic_block("ifcont"));

            emitter
                .builder
                .build_conditional_branch(condition_val, &else_bb, &then_bb);

            emitter.builder.position_at_end(&then_bb);
            let control_if = emit_statement(emitter, *node.block, next_block.clone());
            if control_if == Control::Continue {
                let cont_bb = lazy_cont_bb.force();
                emitter.builder.build_unconditional_branch(&cont_bb);
            }

            emitter.builder.position_at_end(&else_bb);
            let control_else = emit_statement(emitter, *else_block, next_block);
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

            emitter
                .builder
                .build_conditional_branch(condition_val, &cont_bb, &then_bb);

            emitter.builder.position_at_end(&then_bb);
            emit_statement(emitter, *node.block, next_block);
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

    let condition_val = emit_condition_expression_eq(emitter, node.condition_expression);
    emitter
        .builder
        .build_conditional_branch(condition_val, &cont_bb, &then_bb);

    emitter.builder.position_at_end(&then_bb);
    let next_blocks = NextBlock {
        break_block: Some(&cont_bb),
        continue_block: Some(&comp_bb),
    };
    emit_statement(emitter, *node.block, next_blocks);
    emitter.builder.build_unconditional_branch(&comp_bb);

    emitter.builder.position_at_end(&cont_bb);
    Control::Continue
}

fn emit_break_statement(
    emitter: &mut Emitter,
    _node: BreakStatementNode,
    next_block: NextBlock,
) -> Control {
    match next_block.break_block {
        Some(next_block) => emitter.builder.build_unconditional_branch(next_block),
        None => panic!("unexpected break"),
    };
    Control::Break
}

fn emit_continue_statement(
    emitter: &mut Emitter,
    _node: ContinueStatementNode,
    next_block: NextBlock,
) -> Control {
    match next_block.continue_block {
        Some(next_block) => emitter.builder.build_unconditional_branch(next_block),
        None => panic!("unexpected break"),
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
    let scope = Scope::new(emitter);
    emitter.env.push_scope(scope);
    let next_blocks = NextBlock {
        break_block: None,
        continue_block: None,
    };
    emit_statement(emitter, *node.first_statement, next_blocks);
    emitter.builder.build_unconditional_branch(&comp_bb);

    // check condition
    emitter.builder.position_at_end(&comp_bb);
    let condition_val = emit_condition_expression_eq(emitter, node.condition_expression);
    emitter
        .builder
        .build_conditional_branch(condition_val, &cont_bb, &then_bb);

    // emit the block
    emitter.builder.position_at_end(&then_bb);
    let next_blocks = NextBlock {
        break_block: Some(&cont_bb),
        continue_block: Some(&thir_bb),
    };
    emit_statement(emitter, *node.block, next_blocks);
    emitter.builder.build_unconditional_branch(&thir_bb);

    emitter.builder.position_at_end(&thir_bb);
    emit_expression(emitter, node.loop_expression);
    emitter.builder.build_unconditional_branch(&comp_bb);

    emitter.builder.position_at_end(&cont_bb);

    emitter.env.pop_scope();
    Control::Continue
}

fn emit_struct_statement(emitter: &mut Emitter, node: StructStatementNode) -> Control {
    match node {
        StructStatementNode::Definition(node) => {
            let mut field_names: Vec<String> = Vec::new();
            let mut field_types: Vec<BasicTypeEnum> = Vec::new();
            for (field_name, field_type) in node.members {
                field_names.push(field_name);
                let field_type: BasicTypeEnum = emitter.env.get_type_from_string(&field_type);
                field_types.push(field_type);
            }
            let struct_type = emitter.context.struct_type(&field_types, false);
            let struct_value = Struct {
                names: field_names,
                struct_type,
            };
            emitter
                .env
                .insert_new_tag(node.identifier, Tag::Struct(struct_value));
        }
        StructStatementNode::Declare(node) => {
            let declare_variable_node = node.declare_variable_node;
            let identifier: String = declare_variable_node.identifier;
            let value_type = declare_variable_node.value_type;
            let (struct_type, field_names) = match emitter.env.get_tag(&value_type) {
                Some(struct_type) => match struct_type {
                    Tag::Struct(struct_type) => (struct_type.struct_type, struct_type.names),
                },
                None => panic!("unexpected"),
            };
            let alloca = alloca_from_basic_type(emitter, struct_type.into());
            emitter
                .env
                .insert_new_other(identifier.clone(), Other::Variable(alloca));
            for (i, field_name) in field_names.into_iter().enumerate() {
                let field_type = match struct_type.get_field_type_at_index(i as u32) {
                    Some(field_type) => field_type,
                    None => panic!("unexpected"),
                };
                let alloca = alloca_from_basic_type(emitter, field_type);
                let field_identifier = identifier.clone() + "." + &field_name;
                emitter
                    .env
                    .insert_new_other(field_identifier, Other::Variable(alloca));
            }
        }
    }
    Control::Continue
}

fn emit_enum_statement(emitter: &mut Emitter, node: EnumStatementNode) -> Control {
    match node {
        EnumStatementNode::Definition(node) => {
            let _tag = node.tag;
            let enums: Vec<DeclareVariableNode> = node.enums;

            for declare_variable_node in enums {
                emit_declare_statement(
                    emitter,
                    DeclareStatementNode {
                        declare_variable_node,
                    },
                );
            }
            Control::Continue
        }
        EnumStatementNode::Declare(node) => emit_declare_statement(emitter, node),
    }
}

fn emit_switch_statement(emitter: &mut Emitter, node: SwitchStatementNode) -> Control {
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
    let nb = NextBlock {
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

    let condition_val_alloca = emit_expression(emitter, condition_expression);
    let condition_val = load_value(emitter, condition_val_alloca);
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
                    let case_condition_val_alloca =
                        emit_expression(emitter, statement.condition_expression);
                    let case_condition_val = load_value(emitter, case_condition_val_alloca);
                    let cmp_val = emitter.builder.build_int_compare(
                        IntPredicate::EQ,
                        *case_condition_val.as_int_value(),
                        *condition_val.as_int_value(),
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
