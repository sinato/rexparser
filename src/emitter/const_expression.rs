use inkwell::values::BasicValueEnum;

use crate::emitter::*;
use crate::lexer::token::*;
use crate::parser::expression::*;

pub fn emit_const_expression(emitter: &mut Emitter, node: ExpressionNode) -> BasicValueEnum {
    match node {
        ExpressionNode::Token(node) => emit_const_token(emitter, node),
        _ => panic!(format!("TODO {:?}", node)),
    }
}

fn emit_const_token(emitter: &mut Emitter, node: TokenNode) -> BasicValueEnum {
    let default_int_type = emitter.context.i32_type();
    let default_float_type = emitter.context.f32_type();
    match node.token {
        Token::IntNum(val, _) => default_int_type.const_int_from_string(&val, 10).into(),
        Token::FloatNum(val, _) => default_float_type.const_float_from_string(&val).into(),
        Token::Ide(_, _) => panic!(format!("{:?} is not a compile-time constant", node)),
        _ => panic!("TODO"),
    }
}
