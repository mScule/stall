use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::vm::{
    val::{Error, Val},
    VM,
};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_new_map(&mut self) {
        self.vals
            .push(Val::Map(Rc::new(RefCell::new(HashMap::new()))))
    }
    #[inline]
    pub fn op_set_map_val(&mut self) {
        match (self.vals.pop(), self.vals.pop(), self.vals.pop()) {
            (Some(Val::Map(map_ref)), Some(Val::String(key)), Some(val)) => {
                let map_ref_clone = map_ref.clone();
                let mut map = map_ref_clone.borrow_mut();
                map.insert(key, val);
            }
            _ => self.vals.push(Val::Error(Error::from_str("set_map_val"))),
        }
    }
    #[inline]
    pub fn op_get_map_val(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Map(map_ref)), Some(Val::String(key))) => {
                let map_ref_clone = map_ref.clone();
                let map = map_ref_clone.borrow_mut();
                self.vals.push(map[&key].clone());
            }
            _ => self.vals.push(Val::Error(Error::from_str("get_map_val"))),
        }
    }
}
