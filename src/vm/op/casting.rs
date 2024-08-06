use crate::vm::{val::Val, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_to_i64(&mut self) {
        match self.vals.pop() {
            Some(Val::F64(val)) => self.vals.push(Val::I64(val.floor() as i64)),
            Some(Val::String(val)) => match val.parse::<i64>() {
                Ok(val) => self.vals.push(Val::I64(val)),
                _ => panic!("Panic: ToI64 - String"),
            },
            _ => panic!("Panic: ToI64"),
        }
    }
    #[inline]
    pub fn op_to_f64(&mut self) {
        match self.vals.pop() {
            Some(Val::I64(val)) => self.vals.push(Val::I64(val as i64)),
            Some(Val::String(val)) => match val.parse::<f64>() {
                Ok(val) => self.vals.push(Val::F64(val)),
                _ => panic!("Panic: ToF64 - String"),
            },
            _ => panic!("Panic: ToF64"),
        }
    }
    #[inline]
    pub fn op_to_string(&mut self) {
        match self.vals.pop() {
            Some(val) => self.vals.push(Val::String(val.to_string())),
            _ => panic!("Panic: ToString"),
        }
    }
}
