use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_goto(&mut self, index: usize) {
        let call = self.calls.peek_last_mut().expect("No call found");

        call.pc = index;
    }
    #[inline]
    pub fn op_if_true_goto(&mut self, index: usize) {
        let call = self.calls.peek_last_mut().expect("No call found");

        match self.vals.pop() {
            Some(Val::Bool(cond)) => {
                if cond {
                    call.pc = index;
                }
            }
            _ => panic!("Panic: If true go to"),
        }
    }
    #[inline]
    pub fn op_if_false_goto(&mut self, index: usize) {
        let call = self.calls.peek_last_mut().expect("No call found");

        match self.vals.pop() {
            Some(Val::Bool(cond)) => {
                if !cond {
                    call.pc = index;
                }
            }
            _ => panic!("Panic: If false go to"),
        }
    }
}
