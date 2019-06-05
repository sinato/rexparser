use crate::lexer::token::BasicType;
use inkwell::values::PointerValue;
use std::collections::HashMap;

pub struct Environment {
    pub variables_stack: Vec<HashMap<String, (PointerValue, BasicType)>>,
}
impl Environment {
    pub fn new() -> Environment {
        let mut variables_stack: Vec<HashMap<String, (PointerValue, BasicType)>> = Vec::new();
        let global_variables: HashMap<String, (PointerValue, BasicType)> = HashMap::new();
        variables_stack.push(global_variables);
        Environment { variables_stack }
    }

    pub fn find(&self, key: &str) -> Option<usize> {
        for (i, variables) in self.variables_stack.iter().rev().enumerate() {
            if variables.contains_key(key) {
                return Some(self.variables_stack.len() - 1 - i);
            }
        }
        None
    }
    pub fn insert(
        &mut self,
        key: String,
        value: (PointerValue, BasicType),
    ) -> Option<(PointerValue, BasicType)> {
        match self.find(&key) {
            Some(idx) => self.variables_stack[idx].insert(key, value),
            None => {
                if let Some(variables) = self.variables_stack.last_mut() {
                    variables.insert(key, value)
                } else {
                    panic!()
                }
            }
        }
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
    pub fn push_scope(&mut self) {
        let variables: HashMap<String, (PointerValue, BasicType)> = HashMap::new();
        self.variables_stack.push(variables);
    }
    pub fn pop_scope(&mut self) -> Option<HashMap<String, (PointerValue, BasicType)>> {
        self.variables_stack.pop()
    }
}
