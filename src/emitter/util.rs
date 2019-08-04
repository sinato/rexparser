use inkwell::types::{AnyTypeEnum, BasicTypeEnum, FloatType, IntType};
use inkwell::values::{BasicValueEnum, FloatValue, InstructionOpcode, IntValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};

use crate::emitter::expression::*;
use crate::emitter::*;
use crate::parser::expression::*;

pub struct Delay<T, F> {
    value: Option<T>,
    func: F,
}
impl<T, F> Delay<T, F>
where
    F: Fn() -> T,
{
    pub fn new(f: F) -> Delay<T, F> {
        Delay {
            value: None,
            func: f,
        }
    }
    pub fn force(&mut self) -> &T {
        if self.value.is_none() {
            self.value = Some((self.func)());
        }
        self.value.as_ref().unwrap()
    }
}

pub fn cast_to_int(emitter: &mut Emitter, int_type: IntType, value: BasicValueEnum) -> IntValue {
    match value {
        BasicValueEnum::IntValue(value) => value,
        BasicValueEnum::FloatValue(value) => {
            let opcode = InstructionOpcode::FPToSI;
            emitter
                .builder
                .build_cast(opcode, value, int_type, "convert")
                .into_int_value()
        }
        _ => panic!(format!("TODO: {:?}", value)),
    }
}

pub fn cast_to_float(
    emitter: &mut Emitter,
    float_type: FloatType,
    value: BasicValueEnum,
) -> FloatValue {
    match value {
        BasicValueEnum::IntValue(value) => {
            let opcode = InstructionOpcode::SIToFP;
            emitter
                .builder
                .build_cast(opcode, value, float_type, "convert")
                .into_float_value()
        }
        BasicValueEnum::FloatValue(value) => value,
        _ => panic!("TODO"),
    }
}

pub fn store_value(emitter: &mut Emitter, value: BasicValueEnum) -> PointerValue {
    let alloca = emitter
        .builder
        .build_alloca(value.get_type(), "store_value");
    match value.get_type() {
        BasicTypeEnum::IntType(_int_type) => {
            value.into_int_value();
            emitter.builder.build_store(alloca, value);
        }
        BasicTypeEnum::FloatType(_float_type) => {
            value.into_float_value();
            emitter.builder.build_store(alloca, value);
        }
        BasicTypeEnum::PointerType(_pointer_type) => {
            value.into_pointer_value();
            emitter.builder.build_store(alloca, value);
        }
        BasicTypeEnum::ArrayType(_array_type) => {
            value.into_array_value();
            emitter.builder.build_store(alloca, value);
        }
        _ => panic!("TODO"),
    };
    alloca
}

pub fn load_value(emitter: &mut Emitter, alloca: PointerValue) -> BasicValueEnum {
    match alloca.get_type().get_element_type() {
        AnyTypeEnum::IntType(_int_type) => BasicValueEnum::IntValue(
            emitter
                .builder
                .build_load(alloca, "alloca")
                .into_int_value(),
        ),
        AnyTypeEnum::FloatType(_float_type) => BasicValueEnum::FloatValue(
            emitter
                .builder
                .build_load(alloca, "alloca")
                .into_float_value(),
        ),
        AnyTypeEnum::PointerType(_pointer_type) => BasicValueEnum::PointerValue(
            emitter
                .builder
                .build_load(alloca, "alloca")
                .into_pointer_value(),
        ),
        AnyTypeEnum::ArrayType(_array_type) => BasicValueEnum::ArrayValue(
            emitter
                .builder
                .build_load(alloca, "alloca")
                .into_array_value(),
        ),
        _ => panic!(format!("TODO {:?}", alloca.get_type().get_element_type())),
    }
}

pub fn to_fn_type(type_enum: BasicTypeEnum, param_types: Vec<BasicTypeEnum>) -> FunctionType {
    match type_enum {
        BasicTypeEnum::IntType(t) => t.fn_type(&param_types, false),
        _ => panic!("TODO"),
    }
}

pub fn to_array_type(type_enum: BasicTypeEnum, size: u32) -> BasicTypeEnum {
    match type_enum {
        BasicTypeEnum::IntType(t) => BasicTypeEnum::ArrayType(t.array_type(size)),
        BasicTypeEnum::ArrayType(t) => BasicTypeEnum::ArrayType(t.array_type(size)),
        _ => panic!(format!("TODO: {:?}", type_enum)),
    }
}

pub fn to_pointer_type(type_enum: BasicTypeEnum) -> BasicTypeEnum {
    match type_enum {
        BasicTypeEnum::IntType(t) => BasicTypeEnum::PointerType(t.ptr_type(AddressSpace::Generic)),
        BasicTypeEnum::ArrayType(t) => {
            BasicTypeEnum::PointerType(t.ptr_type(AddressSpace::Generic))
        }
        _ => panic!("TODO"),
    }
}

pub fn is_assign_operator(operator: &str) -> bool {
    match operator {
        "=" | "+=" => true,
        _ => false,
    }
}

pub fn emit_compare_expression_int(
    emitter: &mut Emitter,
    operator: &str,
    lhs: IntValue,
    rhs: IntValue,
) -> IntValue {
    let fn_value = match emitter.module.get_function(operator) {
        Some(value) => value,
        None => panic!(format!("call of undeclared function {}", operator)),
    };
    let arguments: Vec<BasicValueEnum> = vec![lhs.into(), rhs.into()];
    let func_call_site = emitter
        .builder
        .build_call(fn_value, &arguments, "callcompare");
    let val = func_call_site
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();
    val
}

pub fn emit_condition_expression_eq(
    emitter: &mut Emitter,
    condition_expression: ExpressionNode,
) -> IntValue {
    let condition_val_alloca = emit_expression(emitter, condition_expression);
    let condition_val = load_value(emitter, condition_val_alloca).into_int_value();
    let const_zero = emitter.context.i32_type().const_int(0, false);
    emitter
        .builder
        .build_int_compare(IntPredicate::EQ, condition_val, const_zero, "foreq")
}
