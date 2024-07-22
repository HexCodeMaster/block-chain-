use log::{info, warn};
use thiserror::Error;
use std::collections::HashMap;

pub trait SmartContract {
    fn execute(&self, params: &ContractParams) -> Result<(), ContractError>;
}

#[derive(Debug)]
pub struct ContractParams {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("Invalid parameters provided")]
    InvalidParams,
    #[error("Execution failed")]
    ExecutionFailed, // This variant can be used in the future for error handling
}

pub struct SampleContract {
    state: HashMap<String, String>,
}

impl SampleContract {
    pub fn new() -> Self {
        SampleContract {
            state: HashMap::new(),
        }
    }
}

impl SmartContract for SampleContract {
    fn execute(&self, params: &ContractParams) -> Result<(), ContractError> {
        if params.key.is_empty() || params.value.is_empty() {
            warn!("Invalid parameters: {:?}", params);
            return Err(ContractError::InvalidParams);
        }

        info!("Executing Sample Contract with params: {:?}", params);

        // Simulate contract logic
        let result = self.state.get(&params.key);
        match result {
            Some(existing_value) => {
                info!("Existing value found: {}", existing_value);
                // Here, we would perform more complex contract logic
                Ok(())
            }
            None => {
                warn!("No existing value found for key: {}", params.key);
                Err(ContractError::ExecutionFailed)
            }
        }
    }
}
