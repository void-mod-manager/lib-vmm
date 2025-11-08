use serde::{Deserialize, Serialize};
use thiserror::Error;


/// Error types for the registry
#[derive(Error, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistryError {
    #[error("Invalid id: {0}")]
    InvalidId(String),
    #[error("Duplicate provider id: {0}")]
    ProviderAlreadyExists(String),
    #[error("Duplicate game provider: {0}")]
    GameAlreadyExists(String),
    #[error("Cannot use reserved identifier 'core' for non-core implementations ({0})")]
    ReservedCoreId(String),
    #[error("Cannot find id {0}")]
    NotFound(String),
}
