use std::collections::HashMap;
use std::env;

pub struct ShellState {
    vars: HashMap<String, String>,
}

impl ShellState {
    pub fn new() -> ShellState {
        ShellState {
            vars: env::vars().collect(),
        }
    }
    pub fn get(&self, name: &str) -> Option<&String> {
        self.vars.get(name)
    }
    pub fn set(&mut self, name: &str, value: &str) {
        self.vars.insert(name.to_string(), value.to_string());
    }
}
