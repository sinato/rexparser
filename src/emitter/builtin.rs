use inkwell::context::Context;
use inkwell::IntPredicate;

use crate::emitter::environment::Environment;
use crate::emitter::Emitter;
use std::path;

pub fn emit_builtin() {
    // initialize
    let context = Context::create();
    let module = context.create_module("builtin_module");
    let builder = context.create_builder();
    let environment = Environment::new();
    let mut emitter = Emitter {
        context,
        builder,
        module,
        environment,
    };

    // emit builtin functions
    emit_comp_int(&mut emitter, "eq_int", IntPredicate::EQ);
    emit_comp_int(&mut emitter, "sgt_int", IntPredicate::SGT);
    emit_comp_int(&mut emitter, "slt_int", IntPredicate::SLT);
    emit_and_int(&mut emitter);
    emit_or_int(&mut emitter);

    // output llvm-ir
    let _ = emitter.module.print_to_file(path::Path::new("builtin.ll"));
}

fn emit_comp_int(emitter: &mut Emitter, function_name: &str, operator: IntPredicate) {
    let i32_type = emitter.context.i32_type();
    let function = emitter.module.add_function(
        function_name,
        i32_type.fn_type(&[i32_type.into(), i32_type.into()], false),
        None,
    );

    let entry_bb = function.append_basic_block("entry");
    let then_bb = function.append_basic_block("then");
    let cont_bb = function.append_basic_block("cont");

    emitter.builder.position_at_end(&entry_bb);
    let arg0 = function.get_first_param().unwrap().into_int_value();
    let arg1 = function.get_nth_param(1).unwrap().into_int_value();
    let cond = emitter.builder.build_int_compare(operator, arg0, arg1, "");
    emitter
        .builder
        .build_conditional_branch(cond, &cont_bb, &then_bb);

    emitter.builder.position_at_end(&then_bb);
    emitter.builder.build_unconditional_branch(&cont_bb);

    emitter.builder.position_at_end(&cont_bb);
    let phi = emitter.builder.build_phi(i32_type, "");
    let entry_val = i32_type.const_int(1, false);
    let then_val = i32_type.const_int(0, false);
    phi.add_incoming(&[(&then_val, &then_bb), (&entry_val, &entry_bb)]);

    emitter
        .builder
        .build_return(Some(&phi.as_basic_value().into_int_value()));
}

fn emit_and_int(emitter: &mut Emitter) {
    let i32_type = emitter.context.i32_type();

    let function = emitter.module.add_function(
        "and_int",
        i32_type.fn_type(&[i32_type.into(), i32_type.into()], false),
        None,
    );

    let entry_bb = function.append_basic_block("entry");
    let then_bb = function.append_basic_block("then");
    let cont_bb = function.append_basic_block("cont");

    emitter.builder.position_at_end(&entry_bb);
    let arg0 = function.get_first_param().unwrap().into_int_value();
    let arg1 = function.get_nth_param(1).unwrap().into_int_value();
    let mul = emitter.builder.build_int_mul(arg0, arg1, "mul");

    let const_zero = emitter.context.i32_type().const_int(0, false);
    let cond = emitter
        .builder
        .build_int_compare(IntPredicate::EQ, mul, const_zero, "eq");
    emitter
        .builder
        .build_conditional_branch(cond, &cont_bb, &then_bb);

    emitter.builder.position_at_end(&then_bb);
    emitter.builder.build_unconditional_branch(&cont_bb);

    emitter.builder.position_at_end(&cont_bb);
    let phi = emitter.builder.build_phi(i32_type, "");
    let entry_val = i32_type.const_int(0, false);
    let then_val = i32_type.const_int(1, false);
    phi.add_incoming(&[(&then_val, &then_bb), (&entry_val, &entry_bb)]);

    emitter
        .builder
        .build_return(Some(&phi.as_basic_value().into_int_value()));
}

fn emit_or_int(emitter: &mut Emitter) {
    let i32_type = emitter.context.i32_type();

    let function = emitter.module.add_function(
        "or_int",
        i32_type.fn_type(&[i32_type.into(), i32_type.into()], false),
        None,
    );

    let entry_bb = function.append_basic_block("entry");
    let then_bb = function.append_basic_block("then");
    let cont_bb = function.append_basic_block("cont");

    emitter.builder.position_at_end(&entry_bb);
    let arg0 = function.get_first_param().unwrap().into_int_value();
    let arg1 = function.get_nth_param(1).unwrap().into_int_value();
    let mul = emitter.builder.build_int_add(arg0, arg1, "add");

    let const_zero = emitter.context.i32_type().const_int(0, false);
    let cond = emitter
        .builder
        .build_int_compare(IntPredicate::EQ, mul, const_zero, "eq");
    emitter
        .builder
        .build_conditional_branch(cond, &cont_bb, &then_bb);

    emitter.builder.position_at_end(&then_bb);
    emitter.builder.build_unconditional_branch(&cont_bb);

    emitter.builder.position_at_end(&cont_bb);
    let phi = emitter.builder.build_phi(i32_type, "");
    let entry_val = i32_type.const_int(0, false);
    let then_val = i32_type.const_int(1, false);
    phi.add_incoming(&[(&then_val, &then_bb), (&entry_val, &entry_bb)]);

    emitter
        .builder
        .build_return(Some(&phi.as_basic_value().into_int_value()));
}
