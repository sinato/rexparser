use inkwell::context::Context;
use inkwell::IntPredicate;

use crate::emitter::Emitter;
use std::path;

pub fn emit_builtin(emitter: &mut Emitter) {
    let context = Context::create();
    let module = context.create_module("builtin_module");
    let builder = context.create_builder();

    let i32_type = context.i32_type();

    let function = module.add_function(
        "comp_int",
        i32_type.fn_type(&[i32_type.into(), i32_type.into()], false),
        None,
    );

    let entry_bb = function.append_basic_block("entry");
    let then_bb = function.append_basic_block("then");
    let cont_bb = function.append_basic_block("cont");

    builder.position_at_end(&entry_bb);
    let arg0 = function.get_first_param().unwrap().into_int_value();
    let arg1 = function.get_nth_param(1).unwrap().into_int_value();
    let cond = builder.build_int_compare(IntPredicate::EQ, arg0, arg1, "");
    builder.build_conditional_branch(cond, &cont_bb, &then_bb);

    builder.position_at_end(&then_bb);
    builder.build_unconditional_branch(&cont_bb);

    builder.position_at_end(&cont_bb);
    let phi = builder.build_phi(i32_type, "");
    let entry_val = i32_type.const_int(1, false);
    let then_val = i32_type.const_int(0, false);
    phi.add_incoming(&[(&then_val, &then_bb), (&entry_val, &entry_bb)]);

    builder.build_return(Some(&phi.as_basic_value().into_int_value()));

    let _ = module.print_to_file(path::Path::new("builtin.ll"));
}
