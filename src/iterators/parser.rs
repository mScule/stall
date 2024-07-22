use std::{collections::HashMap, iter::Peekable};

use crate::{iterators::Tokenizer, Func, Op, Val};

use super::tokenizer::Token;

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
    mod_indexes: Vec<String>,
    cur_mod: String,
}

impl<'a> Parser<'a> {
    pub fn from(string: &'a String, mod_indexes: Option<&Vec<String>>, cur_mod: &String) -> Self {
        Self {
            tokens: Tokenizer::from(&string).peekable(),
            mod_indexes: if let Some(mod_indexes) = mod_indexes {
                mod_indexes.clone()
            } else {
                Vec::new()
            },
            cur_mod: cur_mod.clone(),
        }
    }

    fn get_mod_index(&mut self, mod_name: &String) -> usize {
        match self.mod_indexes.iter().position(|name| name.eq(mod_name)) {
            Some(mod_index) => mod_index,
            None => {
                self.mod_indexes.push(mod_name.clone());

                self.mod_indexes.len() - 1
            }
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
                    "get_const" => match (self.tokens.next(), self.tokens.next()) {
                        (Some(Token::String(mod_name)), Some(Token::Number(val_index))) => {
                            let mod_index = self.get_mod_index(&mod_name);
                            let val_index = val_index
                                .parse::<usize>()
                                .expect("get_const $2 has to be usize");

                            Op::GetConst(mod_index, val_index)
                        }
                        (Some(Token::Word(word)), Some(Token::Number(val_index))) => {
                            if word.as_str() != "cur" {
                                panic!("get_const $1 only allows cur keyword");
                            }

                            let mod_index = self.get_mod_index(&self.cur_mod.clone());
                            let val_index = val_index
                                .parse::<usize>()
                                .expect("get_const $2 has to be usize");

                            Op::GetConst(mod_index, val_index)
                        }
                        _ => panic!("get_const $1 has to be string"),
                    },
                    "new_vec" => Op::NewVec,
                    "new_hashmap" => Op::NewHashMap,
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
                    "push" => Op::Push,
                    "get_index" => todo!(),
                    "set_index" => todo!(),
                    _ => panic!("Unsupported op"),
                },
                // End func }
                Some(Token::CBracket) => return Val::Func(func),
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
