//! Traits for reusable, composable CosmWasm modules.

use cosmwasm_std::{Binary, Response, StdError, StdResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

/// A well typed CosmWasm module
///
/// A module must implement instantiate, execute, and query handlers.
/// These handlers may, however, be no-ops.
///
/// Programmers looking to implement reusable CosmWasm modules should create
/// structs that implement Module.
pub trait Module {
    /// The message sent to the module to instantiate its state.
    type InstantiateMsg: for<'a> Deserialize<'a>;
    /// The response returned by the module after instantiating its state.
    type InstantiateResp: Serialize;
    /// The type of transaction messages this module can handle. For modules
    /// that support multiple types of transaction, this will often times be
    /// a sum type.
    type ExecuteMsg: for<'a> Deserialize<'a>;
    /// The type of query messages this module can handle. For modules that
    /// support multiple queries, this will often times be a sum type.
    type QueryMsg: for<'a> Deserialize<'a>;
    /// The response to queries dispatched to the module.
    type QueryResp: Serialize;
    /// The type of errors this module can generate. This must support
    /// conversion from serde_json::Error in order to properly wrap
    /// serialization and deserialization errors. This must implement
    /// Display for easy stringification.
    type Error: Serialize + From<serde_json::Error> + Display;

    /// The instantiate handler for the module. When a Manager with this
    /// module registered is instantiated, this method may be called.
    fn instantiate(&self, msg: Self::InstantiateMsg) -> Result<Self::InstantiateResp, Self::Error>;
    /// The transaction handler for this module. Messages to this contract
    /// will be dispatched by the Manager.
    fn execute(&self, msg: Self::ExecuteMsg) -> Result<Response, Self::Error>;
    /// The query handler for this module. Messages to this contract will be
    /// dispatched by the Manager.
    fn query(&self, msg: Self::QueryMsg) -> Result<Self::QueryResp, Self::Error>;
}

/// A dynamically typed module.
///
/// GenericModules accept JSON values as their messages and return them as
/// their results. Errors returned by GenericModules are strings. This trait
/// was created to enable a simple dynamic dispatch of messages sent to the
/// contract by the `Manager`.
pub trait GenericModule {
    /// A generic implementation of Module::instantiate
    fn instantiate_value(&mut self, msg: &Value) -> Result<Value, String>;
    /// A generic implementation of Module::execute
    fn execute_value(&mut self, msg: &Value) -> Result<Response, String>;
    /// A generic implementation of Module::query
    fn query_value(&self, msg: &Value) -> StdResult<Binary>;
}

/// An implementation of GenericModule for all valid implementations of Module.
impl<T, A, B, C, D, E, F> GenericModule for T
where
    A: for<'de> Deserialize<'de>,
    B: Serialize,
    C: for<'de> Deserialize<'de>,
    D: for<'de> Deserialize<'de>,
    E: Serialize,
    F: Display,
    T: Module<
        InstantiateMsg = A,
        InstantiateResp = B,
        ExecuteMsg = C,
        QueryMsg = D,
        QueryResp = E,
        Error = F,
    >,
{
    fn instantiate_value(&mut self, msg: &Value) -> Result<Value, String> {
        let parsed_msg = serde_json::from_value(msg.clone()).map_err(|e| e.to_string())?;
        let res = self.instantiate(parsed_msg).map_err(|e| e.to_string())?;
        serde_json::to_value(res).map_err(|e| e.to_string())
    }

    fn execute_value(&mut self, msg: &Value) -> Result<Response, String> {
        let parsed_msg = serde_json::from_value(msg.clone()).map_err(|e| e.to_string())?;
        self.execute(parsed_msg).map_err(|e| e.to_string())
    }

    fn query_value(&self, msg: &Value) -> StdResult<Binary> {
        let parsed_msg = serde_json::from_value(msg.clone())
            .map_err(|e| StdError::generic_err(e.to_string()))?;
        let res = self
            .query(parsed_msg)
            .map_err(|e| StdError::generic_err(e.to_string()))?;
        cosmwasm_std::to_binary(&res)
    }
}
