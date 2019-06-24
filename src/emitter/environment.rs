use crate::lexer::token::BasicType;
use inkwell::values::PointerValue;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Struct {
    pub identifier: String,
    pub members: Vec<(String, BasicType)>,
}
impl Struct {
    pub fn find(self, key: &str) -> Option<(u32, BasicType)> {
        let cloned_struct = self.clone();
        for (i, (identifier, val_type)) in cloned_struct.members.into_iter().enumerate() {
            if key == identifier {
                return Some((i as u32, val_type));
            }
        }
        None
    }
}

pub struct Environment {
    pub variables_stack: Vec<HashMap<String, (PointerValue, BasicType)>>,
    pub structs_stack: Vec<HashMap<String, Struct>>,
}
impl Environment {
    pub fn new() -> Environment {
        let mut variables_stack: Vec<HashMap<String, (PointerValue, BasicType)>> = Vec::new();
        let global_variables: HashMap<String, (PointerValue, BasicType)> = HashMap::new();
        variables_stack.push(global_variables);

        let mut structs_stack: Vec<HashMap<String, Struct>> = Vec::new();
        let global_structs: HashMap<String, Struct> = HashMap::new();
        structs_stack.push(global_structs);

        Environment {
            variables_stack,
            structs_stack,
        }
    }
    pub fn push_scope(&mut self) {
        let variables: HashMap<String, (PointerValue, BasicType)> = HashMap::new();
        self.variables_stack.push(variables);

        let structs: HashMap<String, Struct> = HashMap::new();
        self.structs_stack.push(structs);
    }
    pub fn pop_scope(&mut self) {
        self.variables_stack.pop();
        self.structs_stack.pop();
    }
    pub fn insert_new(
        &mut self,
        key: String,
        value: (PointerValue, BasicType),
    ) -> Option<(PointerValue, BasicType)> {
        if let Some(variables) = self.variables_stack.last_mut() {
            if variables.contains_key(&key) {
                panic!(format!("redefinition of {}", key))
            }
            variables.insert(key, value)
        } else {
            panic!()
        }
    }
    pub fn get(&self, key: &str) -> Option<(PointerValue, BasicType)> {
        let variables_stack = self.variables_stack.clone();
        for mut variables in variables_stack.into_iter().rev() {
            if let Some(value) = variables.remove(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn insert_new_struct(&mut self, key: String, value: Struct) -> Option<Struct> {
        if let Some(structs) = self.structs_stack.last_mut() {
            if structs.contains_key(&key) {
                panic!(format!("redefinition of {}", key))
            }
            structs.insert(key, value)
        } else {
            panic!()
        }
    }
    pub fn get_struct(&self, key: &str) -> Option<Struct> {
        let structs_stack = self.structs_stack.clone();
        for mut structs in structs_stack.into_iter().rev() {
            if let Some(value) = structs.remove(key) {
                return Some(value);
            }
        }
        None
    }
}
