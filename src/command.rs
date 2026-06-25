use crate::error::CommandError;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    LockDoors,
    UnlockDoors,
    RequestVehicleHealth,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub command_id: String,
    pub vehicle_id: String,
    pub command_type: CommandType,
    pub issued_at: SystemTime,
    pub deadline: SystemTime,
}

impl Command {
    pub fn new(
        command_id: impl Into<String>,
        vehicle_id: impl Into<String>,
        command_type: CommandType,
        ttl: Duration,
    ) -> Self {
        let issued_at = SystemTime::now();

        Self {
            command_id: command_id.into(),
            vehicle_id: vehicle_id.into(),
            command_type,
            issued_at,
            deadline: issued_at + ttl,
        }
    }

    pub fn validate(&self) -> Result<(), CommandError> {
        if self.command_id.trim().is_empty() {
            return Err(CommandError::MissingCommandId);
        }

        if self.vehicle_id.trim().is_empty() {
            return Err(CommandError::MissingVehicleId);
        }

        if self.deadline <= SystemTime::now() {
            return Err(CommandError::Expired);
        }

        Ok(())
    }

    pub fn expired(
        command_id: impl Into<String>,
        vehicle_id: impl Into<String>,
        command_type: CommandType,
    ) -> Self {
        let issued_at = SystemTime::now() - Duration::from_secs(60);

        Self {
            command_id: command_id.into(),
            vehicle_id: vehicle_id.into(),
            command_type,
            issued_at,
            deadline: SystemTime::now() - Duration::from_secs(1),
        }
    }
}
