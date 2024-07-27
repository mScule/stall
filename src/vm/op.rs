use super::val::Val;

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
