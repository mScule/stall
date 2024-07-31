mod parser;
mod tokenizer;

use std::collections::HashMap;

use parser::Parser;

use crate::vm::val::Val;

pub struct Reader<'a> {
    funcs: &'a mut HashMap<String, Val>,
}

impl<'a> Reader<'a> {
    pub fn new(funcs: &'a mut HashMap<String, Val>) -> Self {
        Self { funcs }
    }

    /// This function reads an sfuncs string and constructs function literals to referenced funcs hashmap.
    /// * `prefix` - Optional prefix for the function name.
    /// * `text` - The string to parse as sfuncs.
    pub fn read(&mut self, prefix: Option<&str>, text: &str) {
        let parser = Parser::new(text);

        for (name, func) in parser {
            self.funcs.insert(
                match prefix {
                    Some(prefix) => format!("{}{}", prefix, name),
                    _ => name,
                },
                func,
            );
        }
    }
}
