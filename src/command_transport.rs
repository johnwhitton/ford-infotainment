use crate::{command::Command, error::CommandError, event::CommandAcknowledgement};

pub trait CommandTransport {
    fn submit_command(
        &self,
        command: Command,
    ) -> impl std::future::Future<Output = Result<CommandAcknowledgement, CommandError>> + Send;
}
