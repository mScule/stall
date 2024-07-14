mod collections;

use collections::Stack;
use std::{collections::HashMap, time::Instant};

#[derive(Clone)]
enum Op {
    GetConst(usize, usize),

    StartScope,
    EndScope,
    DefVar,
    SetVar(usize, usize),
    GetVar(usize, usize),

    Add,
    Sub,
    Mul,
    Div,

    Concat,

    Call,
    Return,

    GoTo(usize),
    IfTrueGoTo(usize),
    IfFalseGoTo(usize),

    Gte,
    Lte,
    Gt,
    Lt,
    Eq,
    Not,

    ToI64,
    ToF64,
    ToString,

    Push,
    GetIndex,
    SetIndex,
}

#[derive(Clone)]
enum Val {
    None,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Vec(Vec<Val>),
    HashMap(HashMap<Val, Val>),
    Func(Func),
    //Closure()
}

impl Val {
    fn to_string(&self) -> String {
        match self {
            Self::Bool(bool) => String::from(if *bool { "true" } else { "false" }),
            Self::I64(i64) => i64.to_string(),
            Self::F64(f64) => f64.to_string(),
            Self::None => String::from("none"),
            Self::String(val) => val.to_string(),
            Self::Vec(val) => format!("vec@{:p}", val),
            Self::HashMap(val) => format!("hashmap@{:p}", val),
            Self::Func(val) => format!("func@{:p}", val),
        }
    }
}

type Func = Vec<Op>;

struct Call {
    pc: usize,
    func: Func,
}

impl Call {
    fn from(func: Func) -> Self {
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

struct VM {
    status: Status,
    mods: Vec<Vec<Val>>,
    scopes: Stack<Vec<Val>>,
    calls: Stack<Call>,
    vals: Stack<Val>,
}

struct BinOp<T>(T, T);
struct TerOp<T>(T, T, T);

impl VM {
    pub fn new(mods: Vec<Vec<Val>>, main: Func) -> Self {
        Self {
            status: Status::Run,
            mods,
            scopes: Stack::from(Vec::from([Vec::new()])),
            calls: Stack::from(Vec::from([Call::from(main)])),
            vals: Stack::new(),
        }
    }
    pub fn run(&mut self) {
        let start_time = Instant::now();
        let mut i = 1;

        loop {
            i += 1;
            match self.status {
                Status::Run => self.eval(),
                _ => {
                    break;
                }
            }
        }

        let duration = start_time.elapsed();

        self.dump_vals();
        println!("Iterations:\t{}", i);
        println!("Time elapsed:\t{:?}", duration);
    }
    fn dump_vals(&self) {
        let vals = &self.vals;

        print!("vals: [ ");
        vals.for_each(|val| print!("({}) ", val.to_string()));
        println!("]");
    }
    fn eval(&mut self) {
        let call = self.calls.peek_last_mut().unwrap();
        let op = call.next().unwrap().clone();

        match op {
            Op::StartScope => {
                self.scopes.push(Vec::new());
            }
            Op::EndScope => {
                self.scopes.pop();
            }
            Op::GetConst(mod_index, const_index) => match self.mods.get(mod_index) {
                Some(consts) => match consts.get(const_index) {
                    Some(val) => self.vals.push(val.clone()),
                    _ => panic!("Panic: Get const index"),
                },
                _ => panic!("Panic: Get const mod"),
            },
            Op::DefVar => match self.vals.pop() {
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
            Op::Add => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a + b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a + b)),
                _ => panic!("Panic: Add"),
            },
            Op::Sub => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a - b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a - b)),
                _ => panic!("Panic: Add"),
            },
            Op::Mul => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a * b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a * b)),
                _ => panic!("Panic: Add"),
            },
            Op::Div => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a / b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::F64(a / b)),
                _ => panic!("Panic: Add"),
            },
            Op::Concat => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::String(mut a)), Some(Val::String(b))) => {
                    a.push_str(&b);
                    self.vals.push(Val::String(a));
                }
                _ => panic!("Panic: Concat"),
            },
            Op::Gte => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a >= b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a >= b)),
                _ => panic!("Panic: Gte"),
            },
            Op::Lte => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a <= b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a <= b)),
                _ => panic!("Panic: Lte"),
            },
            Op::Gt => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a > b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a > b)),
                _ => panic!("Panic: Gt"),
            },
            Op::Lt => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a < b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a < b)),
                _ => panic!("Panic: Lt"),
            },
            Op::Call => match self.vals.pop() {
                Some(Val::Func(func)) => self.calls.push(Call::from(func)),
                _ => panic!("Panic: Call"),
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
            Op::Return => {
                self.calls.pop();

                if self.calls.len() == 0 {
                    self.status = Status::End;
                }
            }
            Op::Eq => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(val), Some(Val::None))
                | BinOp(Some(Val::None), Some(val)) => match val {
                    Val::None => self.vals.push(Val::Bool(true)),
                    _ => self.vals.push(Val::Bool(false)),
                },
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a == b)),
                BinOp(Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a == b)),
                BinOp(Some(Val::Bool(a)), Some(Val::Bool(b))) => self.vals.push(Val::Bool(a == b)),
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
            Op::Push => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::Vec(mut vec)), Some(val)) => {
                    vec.push(val);
                    self.vals.push(Val::Vec(vec));
                }
                _ => panic!("Panic: Push"),
            },
            Op::SetIndex => match TerOp(self.vals.pop(), self.vals.pop(), self.vals.pop()) {
                TerOp(Some(Val::Vec(mut vec)), Some(Val::I64(index)), Some(val)) => {
                    vec[index as usize] = val;
                    self.vals.push(Val::Vec(vec));
                }
                _ => panic!("Panic: SetIndex"),
            },
            Op::GetIndex => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::Vec(vec)), Some(Val::I64(index))) => {
                    match vec.get(index as usize) {
                        Some(val) => self.vals.push(val.clone()),
                        _ => panic!("Panic: GetIndex - Bad index"),
                    }
                }
                _ => panic!("Panic: GetIndex"),
            },
        }
    }
}

fn main() {
    let mut vm = VM::new(
        vec![vec![
            Val::I64(0),
            Val::I64(1),
            Val::Vec(vec![Val::I64(10), Val::F64(20.40)]),
            Val::String("Hello there".to_string()),
        ]],
        Vec::from([
            Op::GetConst(0, 1),
            Op::GetConst(0, 3),
            Op::GetConst(0, 1),
            Op::GetConst(0, 2),
            Op::SetIndex,
            Op::GetIndex,
            Op::Return,
        ]),
    );

    vm.run();
}
