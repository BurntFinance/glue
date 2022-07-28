use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait Module {
    type State;
    type InstantiateMsg: for<'a> Deserialize<'a>;
    type ExecuteMsg: for<'a> Deserialize<'a>;
    type ExecuteResp: Serialize;
    type QueryMsg: for<'a> Deserialize<'a>;
    type QueryResp: Serialize;
    type Error: Serialize + From<serde_json::Error>;

    fn new() -> Self;

    fn execute(&self, msg: Self::ExecuteMsg) -> Result<Self::ExecuteResp, Self::Error>;
    fn query(&self, msg: Self::QueryMsg) -> Result<Self::QueryResp, Self::Error>;
}

pub trait GenericModule {
    fn instantiate_value(&mut self, msg: &Value);
    fn execute_value(&mut self, msg: &Value) -> Result<Value, String>;
    fn query_value(&self, msg: &Value) -> Result<Value, String>;
}

// pub fn make_generic<M, R, E, IF, OF>(f: IF) -> OF
// where
//     E: From<serde_json::Error> + Serialize,
//     IF: Fn(M) -> Result<R, E>,
//     OF: Fn(Value) -> Result<String, Value>,
// {
//     |msg: Value| -> Result<String, Value> {
//         let parsed_msg = serde_json::from_value(msg).map_err(|e| e.into())?
//         let res = f(parsed_msg)?
//         serde_json::to_string()
//     }
// }
