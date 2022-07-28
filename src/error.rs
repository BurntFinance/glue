use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("module {module:?} already registered")]
    ModuleAlreadyRegistered { module: String },

    #[error("error executing module {module:?}: {err:?}")]
    ExecutionError { module: String, err: String },

    #[error("error querying module {module:?}: {err:?}")]
    QueryError { module: String, err: String },

    #[error("error parsing request: {msg:?}")]
    ParseError { msg: Option<String> },

    #[error("module {module:?} not found")]
    NotFoundError { module: String },
}
