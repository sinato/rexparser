use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{GlobalValue, PointerValue};
use std::collections::HashMap;

use crate::emitter::util::*;
use crate::emitter::Emitter;

#[derive(Debug, PartialEq, Clone)]
pub struct Struct {
    pub names: Vec<String>,
    pub struct_type: StructType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub scopes: Vec<Scope>,
}
impl Environment {
    pub fn new() -> Environment {
        let scopes: Vec<Scope> = Vec::new();
        Environment { scopes }
    }
    pub fn push_scope(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    pub fn insert_new_other(&mut self, key: String, value: Other) -> Option<Other> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.other_stack.contains_key(&key) {
                panic!(format!("redefinition of {}", key))
            }
            scope.other_stack.insert(key, value)
        } else {
            panic!()
        }
    }
    pub fn get_other(&self, key: &str) -> Option<Other> {
        let scopes = self.scopes.clone();
        for mut variables in scopes.into_iter().rev() {
            if let Some(value) = variables.other_stack.remove(key) {
                return Some(value);
            }
        }
        None
    }
    pub fn insert_new_tag(&mut self, key: String, value: Tag) -> Option<Tag> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.tag_stack.contains_key(&key) {
                panic!(format!("redefinition of {}", key))
            }
            scope.tag_stack.insert(key, value)
        } else {
            panic!()
        }
    }
    pub fn get_tag(&self, key: &str) -> Option<Tag> {
        let scopes = self.scopes.clone();
        for mut variables in scopes.into_iter().rev() {
            if let Some(value) = variables.tag_stack.remove(key) {
                return Some(value);
            }
        }
        None
    }
    pub fn get_type_from_string(&self, type_string: &str) -> BasicTypeEnum {
        // parse [] (ex. int[3][2])
        let mut vec: Vec<&str> = type_string
            .split(|c: char| c == '[' || c == ']')
            .filter(|v| !v.is_empty())
            .rev()
            .collect();
        let type_string: &str = vec.pop().unwrap();

        // parse * (ex. int*, float**,...)
        let type_string_split: Vec<&str> = type_string.split("*").collect();
        let pointer_depth = type_string_split.len() - 1;
        let type_string = type_string_split
            .get(0)
            .expect("expect at least one element");

        let mut basic_type = match self.get_other(type_string) {
            Some(other) => match other {
                Other::Type(t) => t,
                _ => panic!(format!("{} is not a type", type_string)),
            },
            None => panic!(format!("type {} is not exist", type_string)),
        };

        for val in vec.into_iter().rev() {
            basic_type = to_array_type(basic_type, val.parse::<u32>().unwrap());
        }

        for _ in 0..pointer_depth {
            basic_type = to_pointer_type(basic_type);
        }
        basic_type
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub other_stack: HashMap<String, Other>,
    pub tag_stack: HashMap<String, Tag>,
}
impl Scope {
    pub fn new(emitter: &mut Emitter) -> Scope {
        let mut other_stack: HashMap<String, Other> = HashMap::new();
        let default_int_type = BasicTypeEnum::IntType(emitter.context.i32_type());
        let default_char_type = BasicTypeEnum::IntType(emitter.context.i8_type());
        let default_float_type = BasicTypeEnum::FloatType(emitter.context.f32_type());
        other_stack.insert(String::from("int"), Other::Type(default_int_type));
        other_stack.insert(String::from("char"), Other::Type(default_char_type));
        other_stack.insert(String::from("float"), Other::Type(default_float_type));

        let tag_stack: HashMap<String, Tag> = HashMap::new();

        Scope {
            other_stack,
            tag_stack,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Other {
    Variable(PointerValue),
    Type(BasicTypeEnum),
    Global(GlobalValue),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Struct(Struct),
}
