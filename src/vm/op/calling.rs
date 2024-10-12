use crate::vm::{
    call::Call,
    val::{Error, Val},
    Status, VM,
};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_call_api(&mut self, key: String) {
        match self.api.get(key.as_str()) {
            Some(func) => func(self),
            _ => self.vals.push(Val::Error(Error::from_str("call_sys"))),
        }
    }
    #[inline]
    pub fn op_call_func(&mut self) {
        match self.vals.pop() {
            Some(Val::Func(func)) => self.calls.push(Call::from(func)),
            _ => self.vals.push(Val::Error(Error::from_str("call_func"))),
        }
    }
    #[inline]
    pub fn op_return_call(&mut self) {
        self.calls.pop();

        if self.calls.len() == 0 {
            self.status = Status::End;
        }
    }
}
