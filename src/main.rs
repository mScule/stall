mod collections;
mod iterators;

use collections::Stack;
use iterators::{Parser, Tokenizer};
use std::{collections::HashMap, time::Instant};

#[derive(Clone, Debug)]
pub enum Op {
    GetConst(usize, usize),

    NewVec,
    NewHashMap,

    NewScope,
    EndScope,
    NewVar,
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

#[derive(Clone, Debug)]
pub enum Val {
    None,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Vec(Vec<Val>),
    HashMap(HashMap<Val, Val>),
    Func(Func),
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

pub type Func = Vec<Op>;
pub type Mod = Vec<Val>;

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
    mods: Vec<Mod>,
    scopes: Stack<Vec<Val>>,
    calls: Stack<Call>,
    vals: Stack<Val>,
}

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
            Op::NewScope => {
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
            Op::NewHashMap => self.vals.push(Val::HashMap(HashMap::new())),
            Op::NewVec => self.vals.push(Val::Vec(Vec::new())),
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
            Op::Eq => match (self.vals.pop(), self.vals.pop()) {
                (Some(val), Some(Val::None)) | (Some(Val::None), Some(val)) => match val {
                    Val::None => self.vals.push(Val::Bool(true)),
                    _ => self.vals.push(Val::Bool(false)),
                },
                (Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a == b)),
                (Some(Val::F64(a)), Some(Val::F64(b))) => self.vals.push(Val::Bool(a == b)),
                (Some(Val::Bool(a)), Some(Val::Bool(b))) => self.vals.push(Val::Bool(a == b)),
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
            Op::Push => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::Vec(mut vec)), Some(val)) => {
                    vec.push(val);
                    self.vals.push(Val::Vec(vec));
                }
                _ => panic!("Panic: Push"),
            },
            Op::SetIndex => match (self.vals.pop(), self.vals.pop(), self.vals.pop()) {
                (Some(Val::Vec(mut vec)), Some(Val::I64(index)), Some(val)) => {
                    vec[index as usize] = val;
                    self.vals.push(Val::Vec(vec));
                }
                _ => panic!("Panic: SetIndex"),
            },
            Op::GetIndex => match (self.vals.pop(), self.vals.pop()) {
                (Some(Val::Vec(vec)), Some(Val::I64(index))) => match vec.get(index as usize) {
                    Some(val) => self.vals.push(val.clone()),
                    _ => panic!("Panic: GetIndex - Bad index"),
                },
                _ => panic!("Panic: GetIndex"),
            },
        }
    }
}

fn main() {
    let input = "
    func {
        |  0 | get_const cur 2  | Define counter
        |  1 | new_var          |

        |  2 | get_const cur 1  | Define message
        |  3 | new_var          |

        |  4 | get_var 0 0      | Check if counter is gte 10
        |  5 | get_const cur 4  |
        |  6 | gte              |
        |  7 | if_false_goto 17 |

        |  8 | get_const cur 1  | Concat string
        |  9 | get_var 0 1      |
        | 10 | concat           |
        | 11 | set_var 0 1      |

        | 12 | get_var 0 0      | Increment counter
        | 13 | get_const cur 3  |
        | 14 | add              |
        | 15 | set_var 0 0      |

        | 16 | goto 4           | Go back to check
        | 17 | get_var 0 1      | Get concatenated message
        | 18 | return           | End program
    }
    \"Hello world!\\n\"
    0i
    1i
    10i
    "
    .to_string();

    let mut mods: Vec<Mod> = Vec::new();
    let parser = Parser::from(&input, None, &"main".to_string());
    let mut main_mod: Mod = Vec::new();

    for val in parser {
        println!("- {:?}", val);
        main_mod.push(val);
    }

    mods.push(main_mod);
    let main_func = match &mods[0][0] {
        Val::Func(main_func) => main_func.clone(),
        _ => panic!("First value of main module has to be the entry point of the program")
    };

    let mut vm = VM::new(
        mods,
        main_func,
    );

    vm.run();
}
