//! A module manager that dynamically dispatches messages sent to a contract
//! to modules registered to it.

use crate::error::Error;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdError, StdResult};
use serde_json::Value;
use serde_json::Value::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::module::GenericModule;
use crate::response::Aggregator;

/// A struct that will dynamically dispatch messages to modules registered
/// within it.
#[derive(Default)]
pub struct Manager {
    modules: HashMap<String, Rc<RefCell<dyn GenericModule>>>,
}

impl Manager {
    /// Create a new Manager with no modules registered to it.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a module, `module`, to the manager under the name `name`.
    /// Entities interacting with the manager can address messages to this
    /// module by wrapping the payload in a root object with a key of `name`
    /// with the associated value, the payload.
    pub fn register(
        &mut self,
        name: String,
        module: Rc<RefCell<dyn GenericModule>>,
    ) -> Result<(), Error> {
        match self.modules.insert(name.clone(), module) {
            Some(_) => Err(Error::ModuleAlreadyRegistered { module: name }),
            None => Ok(()),
        }
    }

    /// Dispatch a JSON-encoded execute message to the appropriate module
    /// registered within the `Manager` instance.
    pub fn execute(
        &mut self,
        deps: &mut DepsMut,
        env: Env,
        info: MessageInfo,
        msg: &str,
    ) -> Result<cosmwasm_std::Response<Binary>, String> {
        let val: Value = serde_json::from_str(msg).map_err(|e| e.to_string())?;
        if let Object(obj) = val {
            let vals: Vec<(String, Value)> = obj.into_iter().collect();
            match &vals[..] {
                [(module_name, payload)] => {
                    if let Some(module) = self.modules.get(module_name) {
                        module
                            .deref()
                            .borrow_mut()
                            .execute_value(deps, env, info, payload)
                            .map(|x| x.into())
                    } else {
                        let err = Error::NotFoundError {
                            module: module_name.to_string(),
                        };
                        Err(format!("{:?}", err))
                    }
                }
                _ => {
                    let err = Error::ParseError {
                        msg: Some("too many module payloads".to_string()),
                    };
                    return Err(format!("{:?}", err));
                }
            }
        } else {
            let err = Error::ParseError { msg: None };
            Err(format!("{:?}", err))
        }
    }

    /// Dispatch a JSON-encoded query message to the appropriate module
    /// registered within the `Manager` instance.
    pub fn query(&mut self, deps: &Deps, env: Env, msg: &str) -> StdResult<Binary> {
        let val: Value =
            serde_json::from_str(msg).map_err(|e| StdError::generic_err(e.to_string()))?;
        if let Object(obj) = val {
            let vals: Vec<(String, Value)> = obj.into_iter().collect();
            match &vals[..] {
                [(module_name, payload)] => {
                    if let Some(module) = self.modules.get(module_name) {
                        module.borrow().query_value(deps, env, payload)
                    } else {
                        let err = Error::NotFoundError {
                            module: module_name.to_string(),
                        };
                        Err(StdError::generic_err(err.to_string()))
                    }
                }
                _ => {
                    let err = Error::ParseError {
                        msg: Some("too many module payloads".to_string()),
                    };
                    Err(StdError::generic_err(err.to_string()))
                }
            }
        } else {
            let err = Error::ParseError { msg: None };
            Err(StdError::generic_err(err.to_string()))
        }
    }

    /// Dispatch JSON-encoded instantiate messages to modules registered within
    /// the Manager.
    pub fn instantiate(
        &mut self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msgs: &str,
    ) -> Result<cosmwasm_std::Response<Binary>, String> {
        let mut aggregator: Aggregator = Aggregator::new();
        let val: Value = serde_json::from_str(msgs).map_err(|e| e.to_string())?;
        if let Object(obj) = val {
            let vals: Vec<(String, Value)> = obj.into_iter().collect();
            for (module_name, payload) in &vals {
                if let Some(module) = self.modules.get(module_name) {
                    let resp = module
                        .deref()
                        .borrow_mut()
                        .instantiate_value(&mut deps, &env, &info, payload)?;
                    aggregator.fold_response(module_name.clone(), resp);
                } else {
                    let err = Error::NotFoundError {
                        module: module_name.to_string(),
                    };
                    return Err(format!("{:?}", err));
                }
            }
            Ok(aggregator.aggregate())
        } else {
            let err = Error::ParseError { msg: None };
            Err(format!("{:?}", err))
        }
    }
}
