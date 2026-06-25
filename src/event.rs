use crate::{
    command::{Command, CommandType},
    error::CommandError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandStatus {
    Accepted,
    Rejected,
    Blocked,
    Executed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandAcknowledgement {
    pub command_id: String,
    pub vehicle_id: String,
    pub command_type: String,
    pub status: CommandStatus,
    pub reason: Option<String>,
}

impl CommandAcknowledgement {
    pub fn accepted(command: &Command) -> Self {
        Self::new(command, CommandStatus::Accepted, None)
    }

    pub fn executed(command: &Command) -> Self {
        Self::new(command, CommandStatus::Executed, None)
    }

    pub fn rejected(command: &Command, err: CommandError) -> Self {
        Self::new(command, CommandStatus::Rejected, Some(err.to_string()))
    }

    pub fn blocked(command: &Command, err: CommandError) -> Self {
        Self::new(command, CommandStatus::Blocked, Some(err.to_string()))
    }

    pub fn failed(command: &Command, err: CommandError) -> Self {
        Self::new(command, CommandStatus::Failed, Some(err.to_string()))
    }

    fn new(command: &Command, status: CommandStatus, reason: Option<String>) -> Self {
        Self {
            command_id: command.command_id.clone(),
            vehicle_id: command.vehicle_id.clone(),
            command_type: command_type_name(&command.command_type).to_string(),
            status,
            reason,
        }
    }
}

fn command_type_name(command_type: &CommandType) -> &'static str {
    match command_type {
        CommandType::LockDoors => "LockDoors",
        CommandType::UnlockDoors => "UnlockDoors",
        CommandType::RequestVehicleHealth => "RequestVehicleHealth",
    }
}
