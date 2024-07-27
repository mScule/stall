mod collections;
mod vals;
mod vm;

use std::collections::HashMap;

use vals::mod_builder::ModBuilder;
use vals::mod_index::ModIndex;
use vm::sys_api::SysFunc;
use vm::val::Val;
use vm::VM;

fn main() {
    let mut mod_index = ModIndex::new();
    let mut mod_builder = ModBuilder::new(&mut mod_index);

    let mut vm = VM::new(
        Vec::from([
            mod_builder.build_mod(
                &"funcs".to_string(),
                &r#"
                    func {
                        new_scope
                        new_var

                        get_var 0 0
                        "Printing: "
                        concat

                        call_sys "std/print"

                        end_scope
                        return
                    }
                "#
                .to_string(),
            ),
            mod_builder.build_mod(
                &"main".to_string(),
                &r#"
                    func {
                        "Hello world!\n"
                        get_mod_val "funcs" 0
                        call_func
                        return
                    }
                "#
                .to_string(),
            ),
        ]),
        HashMap::from([
            ("std/print".to_string(), std_print as SysFunc),
            ("std/val_dump".to_string(), std_val_dump as SysFunc),
        ]),
    );

    vm.start(mod_index.get_index_of_mod(&"main".to_string()), 0);
}

fn std_print(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => print!("{}", val.to_string()),
        _ => (),
    }
}

fn std_val_dump(vm: &mut VM) {
    match vm.vals.pop() {
        Some(val) => vm.vals.push(Val::String(format!("{:?}", val))),
        _ => (),
    }
}
