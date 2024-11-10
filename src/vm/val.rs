use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::op::Op;

pub type Num = fraction::prelude::Fraction;
pub type Func = Vec<Op>;

#[derive(Clone, Debug)]
pub struct Error {
    pub id: String,
}

impl Error {
    pub fn from_str(str: &str) -> Self {
        Self {
            id: str.to_string(),
        }
    }

    pub fn to_map(&self) -> Val {
        let mut map = HashMap::<String, Val>::new();

        map.insert("id".to_string(), Val::String(self.id.to_string()));

        Val::Map(Rc::new(RefCell::new(map)))
    }
}

#[derive(Clone, Debug)]
pub enum Val {
    None,
    Bool(bool),
    Num(Num),
    String(String),
    List(Rc<RefCell<Vec<Val>>>),
    Map(Rc<RefCell<HashMap<String, Val>>>),
    Func(Rc<Func>),
    Error(Error),
}

impl ToString for Val {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Bool(val) => if *val { "true" } else { "false" }.to_string(),
            Self::Num(val) => format!("{:.5}", val),
            Self::String(val) => val.to_string(),
            Self::List(val) => format!("list@{:p}", val.as_ptr()),
            Self::Map(val) => format!("map@{:p}", val.as_ptr()),
            Self::Func(val) => format!("func@{:p}", val.as_ptr()),
            Self::Error(val) => format!("error@{}", val.id.to_string()),
        }
    }
}
