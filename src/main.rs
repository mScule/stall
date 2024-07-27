mod collections;
mod iterators;

use collections::Stack;
use iterators::Parser;
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

#[derive(Clone, Debug)]
pub enum Op {
    GetModVal(usize, usize),
    GetInlineVal(Val),

    NewVec,
    NewMap,

    NewScope,
    EndScope,

    NewVar,
    SetVar(usize, usize),
    GetVar(usize, usize),

    CallSys(String),
    CallFunc,
    Return,

    GoTo(usize),
    IfTrueGoTo(usize),
    IfFalseGoTo(usize),

    Add,
    Sub,
    Mul,
    Div,

    Concat,

    Gte,
    Lte,
    Gt,
    Lt,
    Eq,
    Not,

    ToI64,
    ToF64,
    ToString,

    PushToVec,
    GetVecVal,
    SetVecVal,

    GetMapVal,
    SetMapVal,

    PopVal,
}

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

impl Val {
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

pub type Func = Vec<Op>;
pub type Mod = Vec<Val>;
pub type SysFunc = fn(vm: &mut VM);
pub type Api = HashMap<String, SysFunc>;

struct Call {
    pc: usize,
    func: Rc<Func>,
}

impl Call {
    fn from(func: Rc<Func>) -> Self {
        Self { pc: 0, func }
    }
    fn next(&mut self) -> Option<&Op> {
        let cur_c = self.pc;

        self.pc += 1;
        self.func.get(cur_c)
    }
}

enum Status {
    Run,
    End,
}

pub struct VM {
    status: Status,
    mods: Vec<Mod>,
    scopes: Stack<Vec<Val>>,
    calls: Stack<Call>,
    pub vals: Stack<Val>,
    api: Api,
}

impl VM {
    pub fn new(mods: Vec<Vec<Val>>, main: Rc<Func>, api: Api) -> Self {
        Self {
            status: Status::Run,
            mods,
            scopes: Stack::from(Vec::from([Vec::new()])),
            calls: Stack::from(Vec::from([Call::from(main)])),
            vals: Stack::new(),
            api,
        }
    }
    pub fn run(&mut self) {
        let start_time = Instant::now();
        let mut i = 0;

        loop {
            match self.status {
                Status::Run => self.eval(),
                _ => {
                    break;
                }
            }
            i += 1;
        }

        let duration = start_time.elapsed();

        println!();
        println!("- Iterations:\t{}", i);
        println!("- Time elapsed:\t{:?}", duration);
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
            Op::CallSys(key) => match self.api.get(&key) {
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
                (Some(Val::Vec(a)), Some(Val::Vec(b))) => self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr())),
                (Some(Val::Map(a)), Some(Val::Map(b))) => self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr())),
                (Some(Val::Func(a)), Some(Val::Func(b))) => self.vals.push(Val::Bool(a.as_ptr() == b.as_ptr())),
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

fn main() {
    let vals = r#"
| A program that creates string "Name: Mike, Age: 18"

func {
    new_vec
    new_var

    get_var 0 0
    new_var

    get_var 0 0
    get_var 0 1
    eq

    call_sys"std/print"

    return
}
    "#
    .to_string();

    let mut mods: Vec<Mod> = Vec::new();
    let parser = Parser::from(&vals, None, &"main".to_string());
    let mut main_mod: Mod = Vec::new();

    for val in parser {
        main_mod.push(val);
    }

    mods.push(main_mod);
    let main_func = match &mods[0][0] {
        Val::Func(main_func) => main_func.clone(),
        _ => panic!("First value of main module has to be the entry point of the program"),
    };

    let mut vm = VM::new(
        mods,
        main_func,
        HashMap::from([
            ("std/print".to_string(), std_print as SysFunc),
            ("std/val_dump".to_string(), std_val_dump as SysFunc),
        ]),
    );

    vm.run();
}

fn std_print(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => print!("{}", val.to_string()),
        _ => (),
    }
}

fn std_val_dump(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => vm.vals.push(Val::String(format!("{:?}", val))),
        _ => (),
    }
}
