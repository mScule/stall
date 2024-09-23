use crate::vm::{val::Val, VM};

use std::io::{self, Write};

pub fn print(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => {
            print!("{}", val.to_string());
            io::stdout().flush().unwrap();
        },
        _ => (),
    }
}

pub fn read_line(vm: &mut VM) {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => vm.vals.push(Val::String(input.trim().to_string())),
        _ => vm.vals.push(Val::None),
    }
}
