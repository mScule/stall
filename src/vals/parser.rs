use std::{iter::Peekable, rc::Rc};

use super::{
    mod_index::ModIndex,
    tokenizer::{Token, Tokenizer},
};

use crate::vm::{
    op::Op,
    val::{Func, Val},
};

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
    mod_index: &'a mut ModIndex,
    cur_mod: String,
}

impl<'a> Parser<'a> {
    pub fn new(string: &'a String, mod_index: &'a mut ModIndex, cur_mod: &String) -> Self {
        Self {
            tokens: Tokenizer::from(&string).peekable(),
            mod_index,
            cur_mod: cur_mod.clone(),
        }
    }

    fn build_func(&mut self) -> Val {
        let mut func: Func = Vec::new();

        // Demand {
        match self.tokens.next() {
            Some(Token::OBracket) => (),
            _ => panic!("Expected {{"),
        }

        loop {
            func.push(match self.tokens.next() {
                Some(Token::Word(op)) => match op.as_str() {
                    "none" => Op::GetInlineVal(Val::None),
                    "true" => Op::GetInlineVal(Val::Bool(true)),
                    "false" => Op::GetInlineVal(Val::Bool(false)),
                    "get_mod_val" => match (self.tokens.next(), self.tokens.next()) {
                        (Some(Token::String(mod_name)), Some(Token::Number(val_index))) => {
                            let mod_index = self.mod_index.get_index_of_mod(&mod_name);
                            let val_index = val_index
                                .parse::<usize>()
                                .expect("get_const $2 has to be usize");

                            Op::GetModVal(mod_index, val_index)
                        }
                        (Some(Token::Word(word)), Some(Token::Number(val_index))) => {
                            if word.as_str() != "cur" {
                                panic!("get_const $1 only allows cur keyword");
                            }

                            let mod_index = self.mod_index.get_index_of_mod(&self.cur_mod.clone());
                            let val_index = val_index
                                .parse::<usize>()
                                .expect("get_const $2 has to be usize");

                            Op::GetModVal(mod_index, val_index)
                        }
                        _ => panic!("get_const $1 has to be string"),
                    },
                    "new_vec" => Op::NewVec,
                    "new_map" => Op::NewMap,
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
                    "add" => Op::Add,
                    "sub" => Op::Sub,
                    "mul" => Op::Mul,
                    "div" => Op::Div,
                    "concat" => Op::Concat,
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
                    "to_int" => Op::ToI64,
                    "to_float" => Op::ToF64,
                    "to_string" => Op::ToString,
                    "push_to_vec" => Op::PushToVec,
                    "get_vec_val" => Op::GetVecVal,
                    "set_vec_val" => Op::SetVecVal,
                    "get_map_val" => Op::GetMapVal,
                    "set_map_val" => Op::SetMapVal,
                    "pop_val" => Op::PopVal,
                    _ => panic!("Unsupported op"),
                },
                // Num literal
                Some(Token::Number(num)) => match self.tokens.next() {
                    Some(Token::Word(type_hint)) => match type_hint.as_str() {
                        "i" => {
                            Op::GetInlineVal(Val::I64(num.parse().expect("Number has to be int")))
                        }
                        "f" => {
                            Op::GetInlineVal(Val::F64(num.parse().expect("Number has to be float")))
                        }
                        _ => panic!("Unrecognized type hint"),
                    },
                    _ => panic!("Unexpected EOF"),
                },
                // String literal
                Some(Token::String(str)) => Op::GetInlineVal(Val::String(str)),
                // End func }
                Some(Token::CBracket) => return Val::Func(Rc::new(func)),
                _ => panic!("Bad token"),
            });
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Val;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.next() {
            // Num literal
            Some(Token::Number(num)) => match self.tokens.next() {
                Some(Token::Word(type_hint)) => match type_hint.as_str() {
                    "i" => Some(Val::I64(num.parse().expect("Number has to be int"))),
                    "f" => Some(Val::F64(num.parse().expect("Number has to be float"))),
                    _ => panic!("Unrecognized type hint"),
                },
                _ => panic!("Unexpected EOF"),
            },
            // String literal
            Some(Token::String(str)) => Some(Val::String(str)),
            Some(Token::Word(word)) => match word.as_str() {
                "none" => Some(Val::None),
                "true" => Some(Val::Bool(true)),
                "false" => Some(Val::Bool(false)),
                "func" => Some(self.build_func()),
                _ => todo!(),
            },
            None => None,
            _ => todo!(),
        }
    }
}
