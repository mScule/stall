use crate::vm::{
    val::{Error, Num, Val},
    VM,
};
use std::str::FromStr;

impl<'a> VM<'a> {
    #[inline]
    pub fn op_to_num(&mut self) {
        match self.vals.pop() {
            Some(Val::String(val)) => match Num::from_str(&val) {
                Ok(val) => self.vals.push(Val::Num(val)),
                _ => self.vals.push(Val::Error(Error::from_str("to_num"))),
            },
            _ => self.vals.push(Val::Error(Error::from_str("to_num"))),
        }
    }
    #[inline]
    pub fn op_to_string(&mut self) {
        match self.vals.pop() {
            Some(val) => self.vals.push(Val::String(val.to_string())),
            _ => self.vals.push(Val::Error(Error::from_str("to_string"))),
        }
    }
}
