
use std::collections::HashMap;
use super::VM;

pub type SysFunc = fn(vm: &mut VM);
pub type SysApi = HashMap<String, SysFunc>;
