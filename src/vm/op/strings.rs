use crate::vm::{val::{Error, Val}, VM};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_concat(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::String(mut a)), Some(Val::String(b))) => {
                a.push_str(&b);
                self.vals.push(Val::String(a));
            }
            _ => self.vals.push(Val::Error(Error::from_str("concat"))),
        }
    }
}
