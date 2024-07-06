mod collections;

use collections::Stack;
use std::{collections::HashMap, time::Instant};

#[derive(Clone)]
enum Op {
    GetConst(String, usize),
    // Variables
    StartScope,
    EndScope,
    DefVar,
    SetVar(usize, usize),
    GetVar(usize, usize),
    // ALU
    Add,
    Sub,
    Mul,
    Div,
    // String
    Concat,
    // Functions
    Call,
    Return,
    // Jumping
    GoTo(usize),
    IfTrueGoTo(usize),
    IfFalseGoTo(usize),
    // Comparison
    Gte,
    Lte,
    Gt,
    Lt,
    // Equality
    Eq,
    Not,
    // "Casting"
    ToI64,
    ToF64,
    ToString,
}

#[derive(Clone)]
enum Val {
    Bool(bool),
    I64(i64),
    F64(f64),
    Ref(Ref),
}

impl Val {
    fn to_string(&self) -> String {
        match self {
            Self::Bool(bool) => format!("{}", if *bool { "true" } else { "false" }),
            Self::I64(i64) => format!("{}", i64),
            Self::F64(f64) => format!("{}", f64),
            Self::Ref(val_ref) => match val_ref {
                Ref::None => format!("none"),
                Ref::String(val) => format!("{}", val),
                Ref::Vec(val) => format!("vec[{}]", val.len()),
                Ref::HashMap(val) => format!("hashmap[{} entries]", val.len()),
                Ref::Func(val) => format!("func[{} ops]", val.len()),
            },
        }
    }
}

#[derive(Clone)]
enum Ref {
    None,
    String(String),
    Vec(Vec<Val>),
    HashMap(HashMap<Val, Val>),
    Func(Func),
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
    mods: HashMap<String, Vec<Val>>,
    scopes: Stack<Vec<Val>>,
    calls: Stack<Call>,
    vals: Stack<Val>,
}

struct BinOp<T>(T, T);

impl VM {
    pub fn new(mods: HashMap<String, Vec<Val>>, main: Func) -> Self {
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
            Op::GetConst(id, index) => match self.mods.get(&id) {
                Some(consts) => match consts.get(index) {
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
                _ => panic!("Panic: Add"),
            },
            Op::Sub => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a - b)),
                _ => panic!("Panic: Add"),
            },
            Op::Mul => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a * b)),
                _ => panic!("Panic: Add"),
            },
            Op::Div => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::I64(a / b)),
                _ => panic!("Panic: Add"),
            },
            Op::Concat => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::Ref(Ref::String(a))), Some(Val::Ref(Ref::String(b)))) => {
                    let mut new = a.clone();
                    new.push_str(&b);
                    self.vals.push(Val::Ref(Ref::String(new)));
                }
                _ => panic!("Panic: Concat"),
            },
            Op::Gte => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a >= b)),
                _ => panic!("Panic: Gte"),
            },
            Op::Lte => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a <= b)),
                _ => panic!("Panic: Lte"),
            },
            Op::Gt => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a > b)),
                _ => panic!("Panic: Gt"),
            },
            Op::Lt => match BinOp(self.vals.pop(), self.vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a < b)),
                _ => panic!("Panic: Lt"),
            },
            Op::Call => match self.vals.pop() {
                Some(Val::Ref(Ref::Func(func))) => self.calls.push(Call::from(func)),
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
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => self.vals.push(Val::Bool(a == b)),
                _ => panic!("Panic: Eq"),
            },
            Op::Not => match self.vals.pop() {
                Some(Val::Bool(val)) => self.vals.push(Val::Bool(!val)),
                _ => panic!("Panic: Not"),
            },
            Op::ToI64 => match self.vals.pop() {
                Some(Val::F64(val)) => self.vals.push(Val::I64(val.floor() as i64)),
                Some(Val::Ref(Ref::String(val))) => match val.parse::<i64>() {
                    Ok(val) => self.vals.push(Val::I64(val)),
                    _ => panic!("Panic: ToI64 - String"),
                },
                _ => panic!("Panic: ToI64"),
            },
            Op::ToF64 => match self.vals.pop() {
                Some(Val::I64(val)) => self.vals.push(Val::I64(val as i64)),
                Some(Val::Ref(Ref::String(val))) => match val.parse::<f64>() {
                    Ok(val) => self.vals.push(Val::F64(val)),
                    _ => panic!("Panic: ToI64 - String"),
                },
                _ => panic!("Panic: ToI64"),
            },
            Op::ToString => match self.vals.pop() {
                Some(val) => self.vals.push(Val::Ref(Ref::String(val.to_string()))),
                _ => panic!("Panic: ToString"),
            },
        }
    }
}

fn main() {
    let mut vm = VM::new(
        HashMap::from([(
            String::from("main"),
            Vec::from([
                Val::Ref(Ref::None),

                Val::Ref(Ref::String(String::from("Jack"))),
                Val::I64(18),

                Val::I64(18),
                Val::Ref(Ref::String(String::from("Hello "))),
                Val::Ref(Ref::String(String::from(". Welcome in!"))),
                Val::Ref(Ref::String(String::from("Wait for "))),
                Val::Ref(Ref::String(String::from(" years"))),
            ]),
        )]),
        Vec::from([
            // let msg = null
            Op::GetConst(String::from("main"), 0),
            Op::DefVar,

            // if 5 < 18 {
            Op::GetConst(String::from("main"), 3),
            Op::GetConst(String::from("main"), 2),
            Op::Lt,
            Op::IfFalseGoTo(18),
            Op::StartScope,

                // msg = "Wait for " + 18 - 5 + " years";
                Op::GetConst(String::from("main"), 7),
                Op::GetConst(String::from("main"), 2),
                Op::GetConst(String::from("main"), 3),
                Op::Sub,
                Op::ToString,
                Op::GetConst(String::from("main"), 6),
                Op::Concat,
                Op::Concat,
                Op::SetVar(1, 0),

            // } else {
            Op::EndScope,
            Op::GoTo(26),
            Op::StartScope,

                // msg = "Hello" + "Jack" + ". Welcome in!";
                Op::GetConst(String::from("main"), 5),
                Op::GetConst(String::from("main"), 1),
                Op::GetConst(String::from("main"), 4),
                Op::Concat,
                Op::Concat,
                Op::SetVar(1, 0),

            // }
            Op::EndScope,

            // msg;
            Op::GetVar(0, 0),
            Op::Return,
        ]),
    );

    vm.run();
}
