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
    pub fn all(&self) -> &HashMap<String, String> {
        &self.vars
    }
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

// $variable zerlegen und info aus env ziehen
pub fn expand(word: &str, state: &ShellState) -> String {
    match word.strip_prefix("$") {
        Some(name) => state.get(name).cloned().unwrap_or_default(),
        None => word.to_string(),
    }
}
