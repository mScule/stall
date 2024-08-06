use crate::vm::{val::Val, VM};

pub fn val_dump(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => vm.vals.push(Val::String(format!("{:?}", val))),
        _ => (),
    }
}
