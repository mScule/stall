use crate::vm::{val::Val, VM};
use fraction::ToPrimitive;
use std::{cell::RefCell, rc::Rc};

impl<'a> VM<'a> {
    #[inline]
    pub fn op_new_vec(&mut self) {
        self.vals.push(Val::Vec(Rc::new(RefCell::new(Vec::new()))))
    }
    #[inline]
    pub fn op_push_to_vec(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Vec(vec_ref)), Some(val)) => {
                let vec_ref_clone = vec_ref.clone();
                let mut vec = vec_ref_clone.borrow_mut();
                vec.push(val);
            }
            _ => panic!("Panic: PushToVec"),
        }
    }
    #[inline]
    pub fn op_set_vec_val(&mut self) {
        match (self.vals.pop(), self.vals.pop(), self.vals.pop()) {
            (Some(Val::Vec(vec_ref)), Some(Val::Num(index)), Some(val)) => {
                let vec_ref_clone = vec_ref.clone();
                let mut vec = vec_ref_clone.borrow_mut();

                match index.to_usize() {
                    Some(index) => vec[index] = val,
                    _ => panic!("Panic: cannot convert value to usize"),
                }
            }
            _ => panic!("Panic: SetIndex"),
        }
    }
    #[inline]
    pub fn op_get_vec_val(&mut self) {
        match (self.vals.pop(), self.vals.pop()) {
            (Some(Val::Vec(vec_ref)), Some(Val::Num(index))) => match index.to_usize() {
                Some(index) => match vec_ref.borrow().get(index as usize) {
                    Some(val) => self.vals.push(val.clone()),
                    _ => panic!("Panic: GetIndex - Bad index"),
                },
                _ => panic!("Panic: Cannot convert value to usize"),
            },
            _ => panic!("Panic: GetIndex"),
        }
    }
}
