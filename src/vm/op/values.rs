use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_get_func(&mut self, key: String) {
        self.vals.push(self.funcs[&key].clone())
    }
    #[inline]
    pub fn op_get_const(&mut self, val: Val) {
        self.vals.push(val.clone());
    }
}
