mod collections;

use collections::Stack;
use std::collections::HashMap;

#[derive(Clone)]
enum Op {
    GetConst(String, usize),
    Add,
    Call,
    Return,
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
            Self::Bool(bool) => format!("Bool({})", if *bool { "true" } else { "false" }),
            Self::I64(i64) => format!("I64({})", i64),
            Self::F64(f64) => format!("F64({})", f64),
            Self::Ref(_) => format!("Ref"),
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

struct VM {
    mods: HashMap<String, Vec<Val>>,
    calls: Stack<Call>,
    vals: Stack<Val>,
}

struct BinOp<T>(T, T);

impl VM {
    fn dump_vals(&self) {
        let vals = &self.vals;

        println!("vals: [");
        vals.for_each(|val| println!("\t{}", val.to_string()));
        println!("]");
    }
    fn eval(&mut self) {
        let VM { mods, calls, vals } = self;

        println!("calls: {}", calls.len());
        let call = calls.peek_last_mut().unwrap();
        let op = call.next().unwrap();

        match op {
            Op::GetConst(id, index) => match mods.get(id) {
                Some(consts) => match consts.get(*index) {
                    Some(val) => vals.push(val.clone()),
                    _ => panic!("Panic: Get const index"),
                },
                _ => panic!("Panic: Get const mod"),
            },
            Op::Add => match BinOp(vals.pop(), vals.pop()) {
                BinOp(Some(Val::I64(a)), Some(Val::I64(b))) => vals.push(Val::I64(a + b)),
                _ => panic!("Panic: Add"),
            },
            Op::Call => match vals.pop() {
                Some(Val::Ref(Ref::Func(func))) => calls.push(Call::from(func)),
                _ => panic!("Panic: Call"),
            },
            Op::Return => {
                calls.pop();
            }
        }
    }
}

fn main() {
    let mut vm = VM {
        mods: HashMap::from([(
            String::from("main"),
            Vec::from([
                Val::I64(10),
                Val::I64(20),
                Val::I64(10),
                Val::I64(20),
                Val::Ref(Ref::Func(Vec::from([Op::Add, Op::Add, Op::Return]))),
                Val::Ref(Ref::Func(Vec::from([Op::Add, Op::Return])))
            ]),
        )]),
        calls: Stack::from(Vec::from([Call::from(Vec::from([
            Op::GetConst(String::from("main"), 0),
            Op::GetConst(String::from("main"), 1),
            Op::GetConst(String::from("main"), 2),
            Op::GetConst(String::from("main"), 3),
            Op::GetConst(String::from("main"), 4),
            Op::Call,
            Op::GetConst(String::from("main"), 5),
            Op::Call,
            Op::Return
        ]))])),
        vals: Stack::new(),
    };

    for _ in 0..14 {
        vm.eval();
        vm.dump_vals();
    }
}
