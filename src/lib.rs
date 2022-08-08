//! A library that enables composability and reuse withing CosmWasm contracts.
//!
//! ## Creating Module
//! To create a reusable module, one must create a struct that implements the
//! [Module][crate::module::Module] trait.
//!
//! ## Using Modules
//!

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
