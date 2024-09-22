use crate::vm::{
    val::{Num, Val},
    VM,
};
use std::str::FromStr;

impl<'a> VM<'a> {
    #[inline]
    pub fn op_to_num(&mut self) {
        match self.vals.pop() {
            Some(Val::String(val)) => match Num::from_str(&val) {
                Ok(val) => self.vals.push(Val::Num(val)),
                _ => panic!("Panic: ToNum - String"),
            },
            _ => panic!("Panic: Num"),
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
