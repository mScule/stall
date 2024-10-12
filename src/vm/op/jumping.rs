use crate::vm::{
    val::{Error, Val},
    VM,
};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_goto(&mut self, index: usize) {
        match self.calls.peek_last_mut() {
            Some(call) => call.pc = index,
            None => self.vals.push(Val::Error(Error::from_str("goto"))),
        }
    }
    #[inline]
    pub fn op_if_true_goto(&mut self, index: usize) {
        match self.calls.peek_last_mut() {
            Some(call) => match self.vals.pop() {
                Some(Val::Bool(cond)) => {
                    if cond {
                        call.pc = index;
                    }
                }
                _ => self
                    .vals
                    .push(Val::Error(Error::from_str("if_true_goto/vals"))),
            },
            None => self
                .vals
                .push(Val::Error(Error::from_str("if_true_goto/call"))),
        }
    }
    #[inline]
    pub fn op_if_false_goto(&mut self, index: usize) {
        match self.calls.peek_last_mut() {
            Some(call) => match self.vals.pop() {
                Some(Val::Bool(cond)) => {
                    if !cond {
                        call.pc = index;
                    }
                }
                _ => self
                    .vals
                    .push(Val::Error(Error::from_str("if_false_goto/pop"))),
            },
            _ => self
                .vals
                .push(Val::Error(Error::from_str("if_false_goto/call"))),
        }
    }
}
