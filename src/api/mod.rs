use super::VM;
use std::collections::HashMap;

pub type ExtFunc = fn(vm: &mut VM);
pub type Api = HashMap<String, ExtFunc>;

#[macro_export]
macro_rules! api {
    ( $( ($key:expr, $value:expr) ),* $(,)? ) => {
        {
            let mut api = Api::new();
            $(
                api.insert($key.to_string(), $value as ExtFunc);
            )*
            api
        }
    };
}
