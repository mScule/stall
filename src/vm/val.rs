use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::op::Op;

pub type Num = fraction::prelude::Fraction;
pub type Func = Vec<Op>;

#[derive(Clone, Debug)]
pub enum Val {
    None,
    Bool(bool),
    Num(Num),
    String(String),
    Vec(Rc<RefCell<Vec<Val>>>),
    Map(Rc<RefCell<HashMap<String, Val>>>),
    Func(Rc<Func>),
}

impl ToString for Val {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Bool(val) => if *val { "true" } else { "false" }.to_string(),
            Self::Num(val) => format!("{:.5}", val),
            Self::String(val) => val.to_string(),
            Self::Vec(val) => format!("vec@{:p}", val.as_ptr()),
            Self::Map(val) => format!("map@{:p}", val.as_ptr()),
            Self::Func(val) => format!("func@{:p}", val.as_ptr()),
        }
    }
}
