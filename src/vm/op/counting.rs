use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_add(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a + b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a + b)),
            _ => panic!("Panic: Add"),
        }
    }
    #[inline]
    pub fn op_sub(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a - b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a - b)),
            _ => panic!("Panic: Sub"),
        }
    }
    #[inline]
    pub fn op_mul(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a * b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a * b)),
            _ => panic!("Panic: Mul"),
        }
    }
    #[inline]
    pub fn op_div(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a / b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a / b)),
            _ => panic!("Panic: Div"),
        }
    }
}
