use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::vm::{val::Val, VM};

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
            _ => panic!("Panic: GetMapVal"),
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
            _ => panic!("Panic: GetMapVal"),
        }
    }
}
