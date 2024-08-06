pub mod calling;
pub mod casting;
pub mod comparison;
pub mod counting;
pub mod jumping;
pub mod maps;
pub mod scopes;
pub mod strings;
pub mod values;
pub mod variables;
pub mod vecs;

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
    CallApi(String),
    CallFunc,
    ReturnCall,

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
