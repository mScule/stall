use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_catch(&mut self) {
        match self.vals.pop() {
            Some(val) => match val {
                Val::Error(error) => self.vals.push(error.to_map()),
                _ => self.vals.push(val),
            },
            _ => (),
        }
    }
}
