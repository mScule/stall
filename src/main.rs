mod api;
mod boot;
mod collections;
mod funcs;
mod vm;

mod std_api;

use api::{Api, ExtFunc};
use boot::Booter;
use vm::VM;

fn main() {
    let api = api![
        ("io/print", std_api::io::print),
        ("debug/dump_vals", std_api::debug::val_dump),
    ];

    let mut vm = VM::new(&api);
    let mut booter = Booter::new(&mut vm);

    booter.boot();
}
