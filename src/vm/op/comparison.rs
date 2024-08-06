use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_gte(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a >= b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a >= b)),
            _ => panic!("Panic: Gte"),
        }
    }
    #[inline]
    pub fn op_lte(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a <= b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a <= b)),
            _ => panic!("Panic: Lte"),
        }
    }
    #[inline]
    pub fn op_gt(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a > b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a > b)),
            _ => panic!("Panic: Gt"),
        }
    }
    #[inline]
    pub fn op_lt(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a < b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a < b)),
            _ => panic!("Panic: Lt"),
        }
    }
    #[inline]
    pub fn op_eq(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(val), Some(Val::None)) | (Some(Val::None), Some(val)) => match val {
                Val::None => self.vals.push(Val::Bool(true)),
                _ => self.vals.push(Val::Bool(false)),
            },
            (Some(Val::Bool(a)), Some(Val::Bool(b))) => self.vals.push(Val::Bool(a == b)),
            (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a == b)),
            (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a == b)),
            (Some(Val::String(a)), Some(Val::String(b))) => self.vals.push(Val::Bool(a.eq(&b))),
            (Some(Val::Vec(a)), Some(Val::Vec(b))) => {
                self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr()))
            }
            (Some(Val::Map(a)), Some(Val::Map(b))) => {
                self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr()))
            }
            (Some(Val::Func(a)), Some(Val::Func(b))) => {
                self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr()))
            }
            _ => panic!("Panic: Eq"),
        }
    }
    #[inline]
    pub fn op_not(&mut self) {
        match self.vals.pop() {
            Some(Val::Bool(val)) => self.vals.push(Val::Bool(!val)),
            _ => panic!("Panic: Not"),
        }
    }
}
