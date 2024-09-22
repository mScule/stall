use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_add(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a + b)),
            _ => panic!("Panic: Add"),
        }
    }
    #[inline]
    pub fn op_sub(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a - b)),
            _ => panic!("Panic: Sub"),
        }
    }
    #[inline]
    pub fn op_mul(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a * b)),
            _ => panic!("Panic: Mul"),
        }
    }
    #[inline]
    pub fn op_div(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Num(a)), Some(Val::Num(b))) => self.vals.push(Val::Num(a / b)),
            _ => panic!("Panic: Div"),
        }
    }
}
