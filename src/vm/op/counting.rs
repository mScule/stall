use crate::vm::{
    val::{Error, Val},
    VM,
};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_add(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a + b)),
            _ => self.vals.push(Val::Error(Error::from_str("add"))),
        }
    }
    #[inline]
    pub fn op_sub(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a - b)),
            _ => self.vals.push(Val::Error(Error::from_str("sub"))),
        }
    }
    #[inline]
    pub fn op_mul(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a * b)),
            _ => self.vals.push(Val::Error(Error::from_str("mul"))),
        }
    }
    #[inline]
    pub fn op_div(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a / b)),
            _ => self.vals.push(Val::Error(Error::from_str("div"))),
        }
    }
}
