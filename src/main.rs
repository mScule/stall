mod boot;
mod collections;
mod funcs;
mod std_sys_api;
mod vm;

use boot::{get_main_path, Booter};
use std_sys_api::STD_SYS_API;
use vm::sys_api::create_sys_api;
use vm::VM;

fn main() {
    let sys_api = create_sys_api([
        STD_SYS_API
    ]);

    let mut vm = VM::new(&sys_api);
    let mut booter = Booter::new(&mut vm);

    if let Some(main_path) = get_main_path() {
        booter.boot(&main_path);
    }
}
