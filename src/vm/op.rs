use super::val::Val;

#[derive(Clone, Debug)]
pub enum Op {
    // Values
    GetFunc(String),
    GetConst(Val),

    // Scopes
    NewScope,
    EndScope,

    // Variables
    NewVar,
    SetVar(usize, usize),
    GetVar(usize, usize),

    // Calling
    CallSys(String),
    CallFunc,
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
    Eq,
    Not,

    // Counting
    Add,
    Sub,
    Mul,
    Div,

    // Strings
    Concat,

    // Casting
    ToI64,
    ToF64,
    ToString,

    // Vecs
    NewVec,
    PushToVec,
    GetVecVal,
    SetVecVal,

    // Maps
    NewMap,
    GetMapVal,
    SetMapVal,
}
