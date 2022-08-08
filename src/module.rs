use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait Module {
    type State;
    type InstantiateMsg: for<'a> Deserialize<'a>;
    type InstantiateResp: Serialize;
    type ExecuteMsg: for<'a> Deserialize<'a>;
    type ExecuteResp: Serialize;
    type QueryMsg: for<'a> Deserialize<'a>;
    type QueryResp: Serialize;
    type Error: Serialize + From<serde_json::Error> + Display;

    fn new() -> Self;

    fn instantiate(&self, msg: Self::InstantiateMsg) -> Result<Self::InstantiateResp, Self::Error>;
    fn execute(&self, msg: Self::ExecuteMsg) -> Result<Self::ExecuteResp, Self::Error>;
    fn query(&self, msg: Self::QueryMsg) -> Result<Self::QueryResp, Self::Error>;
}

pub trait GenericModule {
    fn instantiate_value(&mut self, msg: &Value) -> Result<Value, String>;
    fn execute_value(&mut self, msg: &Value) -> Result<Value, String>;
    fn query_value(&self, msg: &Value) -> Result<Value, String>;
}

impl<T, A, B, C, D, E, F, G, H> GenericModule for T
where
    B: for <'de> Deserialize<'de>,
    C: Serialize,
    D: for <'de> Deserialize<'de>,
    E: Serialize,
    F: for <'de> Deserialize<'de>,
    G: Serialize,
    H: Display,
    T: Module<State = A, InstantiateMsg = B, InstantiateResp = C, ExecuteMsg = D, ExecuteResp = E, QueryMsg = F, QueryResp = G, Error = H>,
{
    fn instantiate_value(&mut self, msg: &Value) -> Result<Value, String> {
        let parsed_msg = serde_json::from_value(msg.clone()).map_err(|e| e.to_string())?;
        let res = self.instantiate(parsed_msg).map_err(|e| e.to_string())?;
        serde_json::to_value(res).map_err(|e| e.to_string())
    }

    fn execute_value(&mut self, msg: &Value) -> Result<Value, String> {
        let parsed_msg = serde_json::from_value(msg.clone()).map_err(|e| e.to_string())?;
        let res = self.execute(parsed_msg).map_err(|e| e.to_string())?;
        serde_json::to_value(res).map_err(|e| e.to_string())
    }

    fn query_value(&self, msg: &Value) -> Result<Value, String> {
        let parsed_msg = serde_json::from_value(msg.clone()).map_err(|e| e.to_string())?;
        let res = self.query(parsed_msg).map_err(|e| e.to_string())?;
        serde_json::to_value(res).map_err(|e| e.to_string())
    }
}
