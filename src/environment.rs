use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum VariableType {
    Integer,
    Text,
}
#[derive(Clone)]
pub struct Variable {
    pub content: String,
    pub vtype: VariableType,
}

// Dict of Variables
pub struct Environment {
    pub vars: HashMap<String, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            vars: HashMap::<String, Variable>::new(),
        };
    }
    pub fn push_var(&mut self, name: String, content: String, vtype: VariableType) {
        self.vars.insert(name, Variable { content, vtype });
    }
    pub fn lookup_var(&self, name: String) -> Option<Variable> {
        if let Some(variable) = self.vars.get(&name) {
            return Some(variable.clone());
            /*if let VariableType::Integer = variable.vtype {
                return Some(variable.clone());
            } else {

            }*/
        } else {
            return None;
        }
    }
}
