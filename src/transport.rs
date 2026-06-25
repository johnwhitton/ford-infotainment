use tokio::sync::{mpsc, oneshot};

use crate::{command::Command, error::CommandError, event::CommandAcknowledgement};

#[derive(Debug)]
pub struct BusMessage {
    pub command: Command,
    pub ack_tx: oneshot::Sender<CommandAcknowledgement>,
}

impl BusMessage {
    pub fn new(command: Command) -> (Self, oneshot::Receiver<CommandAcknowledgement>) {
        let (ack_tx, ack_rx) = oneshot::channel();

        (Self { command, ack_tx }, ack_rx)
    }
}

#[derive(Debug, Clone)]
pub struct InProcessTransport {
    sender: mpsc::Sender<BusMessage>,
}

impl InProcessTransport {
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<BusMessage>) {
        let (sender, receiver) = mpsc::channel(capacity);

        (Self { sender }, receiver)
    }

    pub async fn publish(&self, message: BusMessage) -> Result<(), CommandError> {
        self.sender
            .send(message)
            .await
            .map_err(|_| CommandError::BusSendFailed)
    }
}
