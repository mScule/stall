pub mod op;
pub mod val;

mod call;

use crate::api::Api;
use crate::collections::stack::Stack;

use call::Call;
use op::Op;
use std::collections::HashMap;
use val::Val;

pub enum Status {
    Run,
    End,
}

pub type Funcs = HashMap<String, Val>;

pub struct VM<'a> {
    api: &'a Api,
    funcs: Funcs,

    status: Status,
    scopes: Stack<Vec<Val>>,
    calls: Stack<Call>,
    pub vals: Stack<Val>,
}

impl<'a> VM<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            funcs: Funcs::new(),

            status: Status::Run,
            scopes: Stack::from_vec(Vec::from([Vec::new()])),
            calls: Stack::from_vec(Vec::new()),
            vals: Stack::new(),
        }
    }
    pub fn set_funcs(&mut self, funcs: Funcs) {
        self.funcs = funcs;
    }
    pub fn start(&mut self, func_name: &str) {
        let main = &self.funcs.get(func_name);

        match main {
            Some(Val::Func(main_func)) => self.calls.push(Call::from(main_func.clone())),
            _ => panic!("Function \"{}\" is not avaliable", func_name),
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
            // Values
            Op::GetConst(key) => self.op_get_func(key),
            Op::GetLit(val) => self.op_get_const(val),

            // Scopes
            Op::NewScope => self.op_new_scope(),
            Op::EndScope => self.op_end_scope(),

            // Variables
            Op::NewVar => self.op_new_var(),
            Op::SetVar(offset, index) => self.op_set_var(offset, index),
            Op::GetVar(offset, index) => self.op_get_var(offset, index),

            // Calling
            Op::CallSys(key) => self.op_call_api(key),
            Op::CallFunc => self.op_call_func(),
            Op::ReturnCall => self.op_return_call(),

            // Jumping
            Op::GoTo(index) => self.op_goto(index),
            Op::IfTrueGoTo(index) => self.op_if_true_goto(index),
            Op::IfFalseGoTo(index) => self.op_if_false_goto(index),

            // Comparison
            Op::Gte => self.op_gte(),
            Op::Lte => self.op_lte(),
            Op::Gt => self.op_gt(),
            Op::Lt => self.op_lt(),
            Op::Eq => self.op_eq(),
            Op::Not => self.op_not(),

            // Counting
            Op::Add => self.op_add(),
            Op::Sub => self.op_sub(),
            Op::Mul => self.op_mul(),
            Op::Div => self.op_div(),

            // Strings
            Op::Concat => self.op_concat(),

            // Casting
            Op::ToNum => self.op_to_num(),
            Op::ToString => self.op_to_string(),

            // Vecs
            Op::NewVec => self.op_new_vec(),
            Op::PushToVec => self.op_push_to_vec(),
            Op::SetVecVal => self.op_set_vec_val(),
            Op::GetVecVal => self.op_get_vec_val(),

            // Maps
            Op::NewMap => self.op_new_map(),
            Op::SetMapVal => self.op_set_map_val(),
            Op::GetMapVal => self.op_get_map_val(),
        }
    }
}
