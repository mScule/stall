use crate::vm::{sys_api::SysApiModule, val::Val, VM};

pub const STD_SYS_API: SysApiModule = &[
    ("std/print", print),
    ("std/val_dump", val_dump)
];

fn print(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => print!("{}", val.to_string()),
        _ => (),
    }
}

fn val_dump(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => vm.vals.push(Val::String(format!("{:?}", val))),
        _ => (),
    }
}
