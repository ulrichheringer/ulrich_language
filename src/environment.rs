use std::collections::HashMap;

// Dict of Variables
pub struct Environment {
    pub vars: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            vars: HashMap::<String, String>::new(),
        };
    }
    pub fn push_var(&mut self, name: String, content: String) {
        self.vars.insert(name, content);
    }
    pub fn lookup_var(&self, name: String) -> Option<String> {
        if let Some(variable) = self.vars.get(&name) {
            return Some(variable.to_owned());
        } else {
            return None;
        }
    }
}
