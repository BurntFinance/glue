//! A module manager that dynamically dispatches messages sent to a contract
//! to modules registered to it.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use cosmwasm_std::Response;
use serde_json::{Map, Value};
use serde_json::Value::Object;
use crate::error::Error;

use crate::module::GenericModule;

/// A struct that will dynamically dispatch messages to modules registered
/// within it.
pub struct Manager {
    modules: HashMap<String, Rc<RefCell<dyn GenericModule>>>
}

impl Manager {
    /// Create a new Manager with no modules registered to it.
    pub fn new() -> Self {
        Manager {
            modules: HashMap::new(),
        }
    }

    /// Register a module, `module`, to the manager under the name `name`.
    /// Entities interacting with the manager can address messages to this
    /// module by wrapping the payload in a root object with a key of `name`
    /// with the associated value, the payload.
    pub fn register(&mut self, name: String, module: Rc<RefCell<dyn GenericModule>>) -> Result<(), Error> {
        match self.modules.insert(name.clone(), module) {
            Some(_) => Err(Error::ModuleAlreadyRegistered { module: name }),
            None => Ok(())
        }
    }

    /// Dispatch a JSON-encoded execute message to the appropriate module
    /// registered within the `Manager` instance.
    pub fn execute(&mut self, msg: &str) -> Result<Response, String> {
        let val: Value = serde_json::from_str(msg).map_err(|e| e.to_string())?;
        if let Object(obj) = val {
            let vals: Vec<(String, Value)> = obj.into_iter().collect();
            match &vals[..] {
                [(module_name, payload)] => {
                    if let Some(module) = self.modules.get(module_name) {
                        module.borrow_mut().execute_value(payload)
                    } else {
                        let err = Error::NotFoundError { module: module_name.to_string() };
                        Err(format!("{:?}", err))
                    }
                },
                _ => {
                    let err = Error::ParseError{ msg: Some("too many module payloads".to_string()) };
                    return Err(format!("{:?}", err))
                }
            }
        } else {
            let err = Error::ParseError{ msg: None };
            Err(format!("{:?}", err))
        }
    }

    /// Dispatch a JSON-encoded query message to the appropriate module
    /// registered within the `Manager` instance.
    pub fn query(&mut self, msg: &str) -> Result<Value, String> {
        let val: Value = serde_json::from_str(msg).map_err(|e| e.to_string())?;
        if let Object(obj) = val {
            let vals: Vec<(String, Value)> = obj.into_iter().collect();
            match &vals[..] {
                [(module_name, payload)] => {
                    if let Some(module) = self.modules.get(module_name) {
                        module.borrow_mut().query_value(payload)
                    } else {
                        let err = Error::NotFoundError { module: module_name.to_string() };
                        Err(format!("{:?}", err))
                    }
                },
                _ => {
                    let err = Error::ParseError{ msg: Some("too many module payloads".to_string()) };
                    return Err(format!("{:?}", err))
                }
            }
        } else {
            let err = Error::ParseError{ msg: None };
            Err(format!("{:?}", err))
        }
    }

    /// Dispatch JSON-encoded instantiate messages to modules registered within
    /// the Manager.
    pub fn initialize(&mut self, msgs: &str) -> Result<Value, String> {
        let val: Value = serde_json::from_str(msgs).map_err(|e| e.to_string())?;
        if let Object(obj) = val {
            let vals: Vec<(String, Value)> = obj.into_iter().collect();
            let mut result: Map<String, Value> = Map::new();
            for (module_name, payload) in &vals {
                if let Some(module) = self.modules.get(module_name) {
                    let module_result = module.borrow_mut().instantiate_value(payload)?;
                    result.insert(module_name.clone(), module_result);
                } else {
                    let err = Error::NotFoundError { module: module_name.to_string() };
                    return Err(format!("{:?}", err))
                }
            }
            Ok(Object(result))
        } else {
            let err = Error::ParseError{ msg: None };
            Err(format!("{:?}", err))
        }
    }
}
