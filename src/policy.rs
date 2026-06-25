use std::collections::HashSet;

use crate::{
    command::{Command, CommandType},
    error::CommandError,
};

#[derive(Debug, Clone)]
pub struct VehicleState {
    pub is_moving: bool,
    pub doors_locked: bool,
    pub climate_available: bool,
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            is_moving: false,
            doors_locked: false,
            climate_available: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct PolicyEngine {
    seen_command_ids: HashSet<String>,
    vehicle_state: VehicleState,
}

impl PolicyEngine {
    pub fn new(vehicle_state: VehicleState) -> Self {
        Self {
            seen_command_ids: HashSet::new(),
            vehicle_state,
        }
    }

    pub fn evaluate(&mut self, command: &Command) -> Result<(), CommandError> {
        if self.seen_command_ids.contains(&command.command_id) {
            return Err(CommandError::Duplicate);
        }

        if matches!(command.command_type, CommandType::UnlockDoors) && self.vehicle_state.is_moving
        {
            return Err(CommandError::UnsafeState(
                "cannot unlock doors while vehicle is moving".to_string(),
            ));
        }

        self.seen_command_ids.insert(command.command_id.clone());

        Ok(())
    }
}
