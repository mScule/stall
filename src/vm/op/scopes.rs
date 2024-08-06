use crate::vm::VM;

impl<'a> VM<'a> {
    #[inline]
    pub fn op_new_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    #[inline]
    pub fn op_end_scope(&mut self) {
        self.scopes.pop();
    }
}
