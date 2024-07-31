use std::{iter::Peekable, rc::Rc};

use super::tokenizer::{Token, Tokenizer};

use crate::vm::{
    op::Op,
    val::{Func, Val},
};

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            tokens: Tokenizer::from(&string).peekable(),
        }
    }

    fn build_func(&mut self, key: String) -> (String, Val) {
        let mut func: Func = Vec::new();

        // Demand {
        match self.tokens.next() {
            Some(Token::OBracket) => (),
            _ => panic!("Expected {{"),
        }

        loop {
            func.push(match self.tokens.next() {
                Some(Token::Word(op)) => match op.as_str() {
                    "none" => Op::GetConst(Val::None),
                    "true" => Op::GetConst(Val::Bool(true)),
                    "false" => Op::GetConst(Val::Bool(false)),

                    "get_func" => match self.tokens.next() {
                        Some(Token::String(key)) => Op::GetFunc(key),
                        _ => panic!("get_const $1 has to be string"),
                    },

                    "new_scope" => Op::NewScope,
                    "end_scope" => Op::EndScope,

                    "new_var" => Op::NewVar,
                    "set_var" => match (self.tokens.next(), self.tokens.next()) {
                        (Some(Token::Number(offset)), Some(Token::Number(index))) => {
                            let offset =
                                offset.parse::<usize>().expect("set_var $1 has to be usize");
                            let index = index.parse::<usize>().expect("set_var $2 has to be usize");

                            Op::SetVar(offset, index)
                        }
                        _ => panic!("Bad operands: set_var $1: usize, $2: usize"),
                    },
                    "get_var" => match (self.tokens.next(), self.tokens.next()) {
                        (Some(Token::Number(offset)), Some(Token::Number(index))) => {
                            let offset =
                                offset.parse::<usize>().expect("get_var $1 has to be usize");
                            let index = index.parse::<usize>().expect("get_var $2 has to be usize");

                            Op::GetVar(offset, index)
                        }
                        _ => panic!("Bad operands: get_var $1: usize, $2: usize"),
                    },

                    "call_func" => Op::CallFunc,
                    "call_sys" => match self.tokens.next() {
                        Some(Token::String(key)) => Op::CallSys(key),
                        _ => panic!("Bad operands: call_sys $1: string"),
                    },
                    "return" => Op::Return,

                    "goto" => match self.tokens.next() {
                        Some(Token::Number(index)) => {
                            let index = index.parse::<usize>().expect("goto $1 has to be usize");
                            Op::GoTo(index)
                        }
                        _ => panic!("Bad operands: goto $1: usize"),
                    },
                    "if_true_goto" => match self.tokens.next() {
                        Some(Token::Number(index)) => {
                            let index = index
                                .parse::<usize>()
                                .expect("if_true_goto $1 has to be usize");
                            Op::IfTrueGoTo(index)
                        }
                        _ => panic!("Bad operands: if_true_goto $1: usize"),
                    },
                    "if_false_goto" => match self.tokens.next() {
                        Some(Token::Number(index)) => {
                            let index = index
                                .parse::<usize>()
                                .expect("if_false_goto $1 has to be usize");
                            Op::IfFalseGoTo(index)
                        }
                        _ => panic!("Bad operands: if_false_goto $1: usize"),
                    },

                    "gte" => Op::Gte,
                    "lte" => Op::Lte,
                    "gt" => Op::Gt,
                    "lt" => Op::Lt,
                    "eq" => Op::Eq,
                    "not" => Op::Not,

                    "add" => Op::Add,
                    "sub" => Op::Sub,
                    "mul" => Op::Mul,
                    "div" => Op::Div,

                    "concat" => Op::Concat,

                    "to_i64" => Op::ToI64,
                    "to_f64" => Op::ToF64,
                    "to_string" => Op::ToString,

                    "new_vec" => Op::NewVec,
                    "push_to_vec" => Op::PushToVec,
                    "get_vec_val" => Op::GetVecVal,
                    "set_vec_val" => Op::SetVecVal,

                    "new_map" => Op::NewMap,
                    "get_map_val" => Op::GetMapVal,
                    "set_map_val" => Op::SetMapVal,
                    _ => panic!("Unsupported op"),
                },
                // Num literal
                Some(Token::Number(num)) => match self.tokens.next() {
                    Some(Token::Word(type_hint)) => match type_hint.as_str() {
                        "i64" => Op::GetConst(Val::I64(num.parse().expect("Number has to be int"))),
                        "f64" => {
                            Op::GetConst(Val::F64(num.parse().expect("Number has to be float")))
                        }
                        _ => panic!("Unrecognized type hint"),
                    },
                    _ => panic!("Unexpected EOF"),
                },
                // String literal
                Some(Token::String(str)) => Op::GetConst(Val::String(str)),
                // End func }
                Some(Token::CBracket) => return (key, Val::Func(Rc::new(func))),
                _ => panic!("Bad token"),
            });
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (String, Val);

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.next() {
            Some(Token::Word(word)) => Some(self.build_func(word)),
            None => None,
            _ => todo!(),
        }
    }
}
