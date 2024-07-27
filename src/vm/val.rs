use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::op::Op;

pub type Func = Vec<Op>;

#[derive(Clone, Debug)]
pub enum Val {
    None,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Vec(Rc<RefCell<Vec<Val>>>),
    Map(Rc<RefCell<HashMap<String, Val>>>),
    Func(Rc<Func>),
}

impl ToString for Val {
    fn to_string(&self) -> String {
        match self {
            Self::Bool(bool) => String::from(if *bool { "true" } else { "false" }),
            Self::I64(i64) => i64.to_string(),
            Self::F64(f64) => f64.to_string(),
            Self::None => String::from("none"),
            Self::String(val) => val.to_string(),
            Self::Vec(val) => format!("vec@{:p}", val.as_ptr()),
            Self::Map(val) => format!("map@{:p}", val.as_ptr()),
            Self::Func(val) => format!("func@{:p}", val.as_ptr()),
        }
    }
}
