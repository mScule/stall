use super::VM;
use std::collections::HashMap;

pub type SysFunc = fn(vm: &mut VM);
pub type SysApi = HashMap<String, SysFunc>;

pub type SysApiModule = &'static [(&'static str, SysFunc)];

pub fn create_sys_api<const N: usize>(modules: [SysApiModule; N]) -> SysApi {
    let mut sys_api = SysApi::new();

    for module in modules {
        for (name, func) in module {
            sys_api.insert(name.to_string(), *func);
        }
    }

    sys_api
}
