mod collections;
mod sfuncs;
mod vm;

use vm::sys_api::{SysApi, SysFunc};
use vm::val::Val;
use vm::{Funcs, VM};

fn main() {
    let mut funcs = Funcs::new();
    let mut sfuncs_reader = sfuncs::Reader::new(&mut funcs);

    sfuncs_reader.read(
        Some("libs/greeter.sfuncs/"),
        r#"
            greet {
                | 0| 18i64
                | 1| lte

                | 2| if_false_goto 9
                | 3| "Hello "
                | 4| "\n"
                | 5| concat
                | 6| concat
                | 7| call_sys "std_print"
                | 8| return

                | 9| "You are too young\n"
                |10| call_sys "std_print"
                |11| return
            }
        "#,
    );

    sfuncs_reader.read(
        None,
        r#"
            main {
                "Mike" 19i64
                get_func "libs/greeter.sfuncs/greet"
                call_func

                return
            }
        "#,
    );

    let mut vm = VM::new(
        funcs,
        SysApi::from([
            ("std_print".to_string(), std_print as SysFunc),
            ("std_val_dump".to_string(), std_val_dump as SysFunc),
        ]),
    );

    vm.start("main");
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
