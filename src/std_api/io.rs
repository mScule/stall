use crate::vm::VM;

pub fn print(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => print!("{}", val.to_string()),
        _ => (),
    }
}
