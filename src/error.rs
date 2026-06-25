use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CommandError {
    #[error("command_id is required")]
    MissingCommandId,

    #[error("vehicle_id is required")]
    MissingVehicleId,

    #[error("command deadline has expired")]
    Expired,

    #[error("unsafe vehicle state: {0}")]
    UnsafeState(String),
}
