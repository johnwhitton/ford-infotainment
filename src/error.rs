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

    #[error("duplicate command_id")]
    Duplicate,

    #[error("bus send failed")]
    BusSendFailed,

    #[error("service unavailable")]
    ServiceUnavailable,

    #[error("acknowledgement failed")]
    AckFailed,
}
