use crate::vm::Mod;

use super::{mod_index::ModIndex, parser::Parser};

pub struct ModBuilder<'a> {
    mod_index: &'a mut ModIndex,
}

impl<'a> ModBuilder<'a> {
    pub fn new(mod_index: &'a mut ModIndex) -> Self {
        Self { mod_index }
    }

    /// Builds module from string that contains `.vals` syntax
    pub fn build_mod(&mut self, name: &String, vals_src: &String) -> Mod {
        let parser = Parser::new(&vals_src, &mut self.mod_index, &name);
        let mut result: Mod = Vec::new();

        for val in parser {
            result.push(val);
        }

        return result;
    }
}
