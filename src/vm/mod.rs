pub mod sys_api;
pub mod val;
pub mod op;

mod call;

use crate::collections::stack::Stack;
use call::Call;
use op::Op;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use sys_api::SysApi;
use val::Val;

pub enum Status {
    Run,
    End,
}

pub type Mod = Vec<Val>;

pub struct VM {
    mods: Vec<Mod>,
    sys_api: SysApi,

    status: Status,
    scopes: Stack<Vec<Val>>,
    calls: Stack<Call>,
    pub vals: Stack<Val>,
}

impl VM {
    pub fn new(mods: Vec<Vec<Val>>, sys_api: SysApi) -> Self {
        Self {
            mods,
            sys_api,

            status: Status::Run,
            scopes: Stack::from_vec(Vec::from([Vec::new()])),
            calls: Stack::from_vec(Vec::new()),
            vals: Stack::new(),
        }
    }
    pub fn start(&mut self, mod_index: usize, val_index: usize) {
        let main = &self.mods[mod_index][val_index];

        match main {
            Val::Func(main_func) => self.calls.push(Call::from(main_func.clone())),
            _ => panic!(
                "No function avaliable at mod: {} val: {}",
                mod_index, val_index
            ),
        }

        self.run();
    }
    fn run(&mut self) {
        loop {
            match self.status {
                Status::Run => self.eval(),
                _ => {
                    break;
                }
            }
        }
    }
    fn eval(&mut self) {
        let call = self.calls.peek_last_mut().expect("No call found");
        let op = call.next().expect("Cannot access OP").clone();

        match op {
            Op::NewScope => {
                self.scopes.push(Vec::new());
            }
            Op::EndScope => {
                self.scopes.pop();
            }
            Op::GetModVal(mod_index, func_index) => match self.mods.get(mod_index) {
                Some(funcs) => match funcs.get(func_index) {
                    Some(func) => self.vals.push(func.clone()),
                    _ => panic!("Panic: Get const index"),
                },
                _ => panic!("Panic: Get const mod"),
            },
            Op::GetInlineVal(val) => self.vals.push(val.clone()),
            Op::NewMap => self
                .vals
                .push(Val::Map(Rc::new(RefCell::new(HashMap::new())))),
            Op::NewVec => self.vals.push(Val::Vec(Rc::new(RefCell::new(Vec::new())))),
            Op::NewVar => match self.vals.pop() {
                Some(val) => {
                    let scope = self.scopes.peek_last_mut().unwrap();
                    scope.push(val);
                }
                _ => panic!("Panic: Def var"),
            },
            Op::SetVar(offset, index) => match self.vals.pop() {
                Some(val) => {
                    let scope = self
                        .scopes
                        .peek_mut(self.scopes.len() - 1 - offset)
                        .unwrap();

                    scope[index] = val;
                }
                _ => panic!("Panic: Set var"),
            },
            Op::GetVar(offset, index) => match self.scopes.peek_mut(self.scopes.len() - 1 - offset)
            {
                Some(vars) => match vars.get(index) {
                    Some(val) => self.vals.push(val.clone()),
                    _ => panic!("Panic: Get var - Bad index"),
                },
                _ => panic!("Panic: Get var - Bad offset"),
            },
            Op::Add => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a + b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a + b)),
                _ => panic!("Panic: Add"),
            },
            Op::Sub => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a - b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a - b)),
                _ => panic!("Panic: Sub"),
            },
            Op::Mul => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a * b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a * b)),
                _ => panic!("Panic: Mul"),
            },
            Op::Div => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a / b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a / b)),
                _ => panic!("Panic: Div"),
            },
            Op::Concat => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::String(mut a)), Some(Val::String(b))) => {
                    a.push_str(&b);
                    self.vals.push(Val::String(a));
                }
                _ => panic!("Panic: Concat"),
            },
            Op::Gte => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a >= b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a >= b)),
                _ => panic!("Panic: Gte"),
            },
            Op::Lte => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a <= b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a <= b)),
                _ => panic!("Panic: Lte"),
            },
            Op::Gt => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a > b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a > b)),
                _ => panic!("Panic: Gt"),
            },
            Op::Lt => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a < b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a < b)),
                _ => panic!("Panic: Lt"),
            },
            Op::GoTo(amt) => {
                call.pc = amt;
            }
            Op::IfTrueGoTo(i) => match self.vals.pop() {
                Some(Val::Bool(cond)) => {
                    if cond {
                        call.pc = i;
                    }
                }
                _ => panic!("Panic: If true go to"),
            },
            Op::IfFalseGoTo(i) => match self.vals.pop() {
                Some(Val::Bool(cond)) => {
                    if !cond {
                        call.pc = i;
                    }
                }
                _ => panic!("Panic: If false go to"),
            },
            Op::CallSys(key) => match self.sys_api.get(&key) {
                Some(func) => func(self),
                _ => panic!("Panic: System doesn't have following feature {}", key),
            },
            Op::CallFunc => match self.vals.pop() {
                Some(Val::Func(func)) => self.calls.push(Call::from(func)),
                _ => panic!("Panic: Call"),
            },
            Op::Return => {
                self.calls.pop();

                if self.calls.len() == 0 {
                    self.status = Status::End;
                }
            }
            Op::Eq => match (self.vals.pop(), self.vals.pop()) {
                (Some(val), Some(Val::None)) | (Some(Val::None), Some(val)) => match val {
                    Val::None => self.vals.push(Val::Bool(true)),
                    _ => self.vals.push(Val::Bool(false)),
                },
                (Some(Val::Bool(a)), Some(Val::Bool(b))) => self.vals.push(Val::Bool(a == b)),
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a == b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a == b)),
                (Some(Val::String(a)), Some(Val::String(b))) => self.vals.push(Val::Bool(a.eq(&b))),
                (Some(Val::Vec(a)), Some(Val::Vec(b))) => {
                    self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr()))
                }
                (Some(Val::Map(a)), Some(Val::Map(b))) => {
                    self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr()))
                }
                (Some(Val::Func(a)), Some(Val::Func(b))) => {
                    self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr()))
                }
                _ => panic!("Panic: Eq"),
            },
            Op::Not => match self.vals.pop() {
                Some(Val::Bool(val)) => self.vals.push(Val::Bool(!val)),
                _ => panic!("Panic: Not"),
            },
            Op::ToI64 => match self.vals.pop() {
                Some(Val::F64(val)) => self.vals.push(Val::I64(val.floor() as i64)),
                Some(Val::String(val)) => match val.parse::<i64>() {
                    Ok(val) => self.vals.push(Val::I64(val)),
                    _ => panic!("Panic: ToI64 - String"),
                },
                _ => panic!("Panic: ToI64"),
            },
            Op::ToF64 => match self.vals.pop() {
                Some(Val::I64(val)) => self.vals.push(Val::I64(val as i64)),
                Some(Val::String(val)) => match val.parse::<f64>() {
                    Ok(val) => self.vals.push(Val::F64(val)),
                    _ => panic!("Panic: ToI64 - String"),
                },
                _ => panic!("Panic: ToI64"),
            },
            Op::ToString => match self.vals.pop() {
                Some(val) => self.vals.push(Val::String(val.to_string())),
                _ => panic!("Panic: ToString"),
            },
            Op::PushToVec => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::Vec(vec_ref)), Some(val)) => {
                    let vec_ref_clone = vec_ref.clone();
                    let mut vec = vec_ref_clone.borrow_mut();
                    vec.push(val);
                }
                _ => panic!("Panic: Push"),
            },
            Op::SetVecVal => match (self.vals.pop(), self.vals.pop(), self.vals.pop()) {
                (Some(Val::Vec(vec_ref)), Some(Val::I64(index)), Some(val)) => {
                    let vec_ref_clone = vec_ref.clone();
                    let mut vec = vec_ref_clone.borrow_mut();
                    vec[index as usize] = val;
                }
                _ => panic!("Panic: SetIndex"),
            },
            Op::GetVecVal => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::Vec(vec_ref)), Some(Val::I64(index))) => {
                    match vec_ref.borrow().get(index as usize) {
                        Some(val) => self.vals.push(val.clone()),
                        _ => panic!("Panic: GetIndex - Bad index"),
                    }
                }
                _ => panic!("Panic: GetIndex"),
            },
            Op::SetMapVal => match (self.vals.pop(), self.vals.pop(), self.vals.pop()) {
                (Some(Val::Map(map_ref)), Some(Val::String(key)), Some(val)) => {
                    let map_ref_clone = map_ref.clone();
                    let mut map = map_ref_clone.borrow_mut();
                    map.insert(key, val);
                }
                _ => panic!("Panic: GetMapVal"),
            },
            Op::GetMapVal => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::Map(map_ref)), Some(Val::String(key))) => {
                    let map_ref_clone = map_ref.clone();
                    let map = map_ref_clone.borrow_mut();
                    self.vals.push(map[&key].clone());
                }
                _ => panic!("Panic: GetMapVal"),
            },
            Op::PopVal => {
                self.vals.pop();
            }
        }
    }
}
