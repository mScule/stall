use crate::vm::VM;

impl<'a> VM<'a> {
    #[inline]
    pub fn op_new_var(&mut self) {
        match self.vals.pop() {
            Some(val) => {
                let scope = self.scopes.peek_last_mut().unwrap();
                scope.push(val);
            }
            _ => panic!("Panic: NewVar"),
        }
    }
    #[inline]
    pub fn op_set_var(&mut self, offset: usize, index: usize) {
        match self.vals.pop() {
            Some(val) => {
                let scope = self
                    .scopes
                    .peek_mut(self.scopes.len() - 1 - offset)
                    .unwrap();

                scope[index] = val;
            }
            _ => panic!("Panic: SetVar"),
        }
    }
    #[inline]
    pub fn op_get_var(&mut self, offset: usize, index: usize) {
        match self.scopes.peek_mut(self.scopes.len() - 1 - offset) {
            Some(vars) => match vars.get(index) {
                Some(val) => self.vals.push(val.clone()),
                _ => panic!("Panic: Get var - Bad index"),
            },
            _ => panic!("Panic: Get var - Bad offset"),
        }
    }
}
