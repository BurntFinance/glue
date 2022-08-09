//! A library that enables composability and reuse withing CosmWasm contracts.
//!
//! ## Creating Module
//! To create a reusable module, one must create a struct that implements the
//! [Module][crate::module::Module] trait. Simply define the associated types
//! and provide implementations for [instantiate][crate::module::Module::instantiate],
//! [execute][crate::module::Module::execute], and
//! [query][crate::module::Module::query] and you will have a module ready to
//! use with the manager.
//!
//! By convention, it's acceptable for modules to take references to other
//! modules from their constructors. This allows modules to compose easily.
//!
//! ## Using Modules
//! The [Manager][crate::manager::Manager] is a struct used to dynamically
//! dispatch messages to their corresponding modules. Create a new Manager
//! with [Manager::new][crate::manager::Manager::new] and then register
//! modules for dynamic dispatch with [register][crate::manager::Manager::register].
//!
//! When implementing the entrypoints for contracts built with glue, you can
//! simply call the corresponding functions: `execute`, `query`, and `instantiate`.
//!
//! Entities interacting with your contract may follow a simple convention for
//! addressing messages to a specific module withing your contract. For
//! `execute` and `query` calls, the `Manager` expects an object structured as
//! follows:
//!
//! ```javascript
//! { "module_name": { /* payload object to be sent to the module */ } }
//! ```
//!
//! **NOTE**: The root object must contain a single key. If you attempt to
//! address more than one module in an `execute` call, it will fail.
//!
//! The `Manager` will automatically strip away the root object and forward the
//! payload object to the relevant module. The response object returned by the
//! module will be returned directly.
//!
//! The `instantiate` method is only marginally different. Whereas the calls to
//! `execute` and `query` require that caller to send a root object with only a
//! single key, the object sent to the `instantiate` entrypoint may contain a
//! key for each module registered with the manager, e.g.:
//!
//! ```javascript
//! {
//!   "module_one": { /* payload for module one instantiation */ },
//!   "module_two": { /* payload for module two instantiation */ },
//!   /* and so on */
//! }
//! ```

pub mod error;
pub mod manager;
pub mod module;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
