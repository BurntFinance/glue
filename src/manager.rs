use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use serde_json::Value;
use serde_json::Value::Object;
use crate::error::Error;

use crate::module::GenericModule;

pub struct Manager {
    modules: HashMap<String, Rc<RefCell<dyn GenericModule>>>
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, module: Rc<RefCell<dyn GenericModule>>) -> Result<(), Error> {
        match self.modules.insert(name.clone(), module) {
            Some(_) => Err(Error::ModuleAlreadyRegistered { module: name }),
            None => Ok(())
        }
    }

    pub fn dispatch_execution(&mut self, msg: &str) -> Result<Value, String> {
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

    pub fn dispatch_query(&mut self, msg: &str) -> Result<Value, String> {
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
}
