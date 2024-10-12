use crate::vm::{
    val::{Error, Val},
    VM,
};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_new_var(&mut self) {
        match self.vals.pop() {
            Some(val) => {
                let scope = self.scopes.peek_last_mut().unwrap();
                scope.push(val);
            }
            _ => self.vals.push(Val::Error(Error::from_str("new_var"))),
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
            _ => self.vals.push(Val::Error(Error::from_str("set_var"))),
        }
    }
    #[inline]
    pub fn op_get_var(&mut self, offset: usize, index: usize) {
        match self.scopes.peek_mut(self.scopes.len() - 1 - offset) {
            Some(vars) => match vars.get(index) {
                Some(val) => self.vals.push(val.clone()),
                _ => self.vals.push(Val::Error(Error::from_str("get_var"))),
            },
            _ => self.vals.push(Val::Error(Error::from_str("get_var"))),
        }
    }
}
