use std::sync::Arc;

use primitives::handle::{ConsensusHandleMessage, Handle};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct ConsensusHandle {
    inner: Arc<ConsensusInner>,
}

impl ConsensusHandle {
    pub fn new(tx: UnboundedSender<ConsensusHandleMessage>) -> Self {
        println!("Creating ConsensusHandle"); // debug log (bad practice)

        Self {
            inner: Arc::new(ConsensusInner::new(tx)),
        }
    }
}

impl Handle for ConsensusHandle {
    type Msg = ConsensusHandleMessage;

    fn send(&self, msg: Self::Msg) {
        if let Err(err) = self.inner.to_manager_tx.send(msg) {
            error!(
                error = ?err,
                "Failed to send consensus handle message"
            );
        }
    }
}

#[derive(Debug)]
struct ConsensusInner {
    to_manager_tx: UnboundedSender<ConsensusHandleMessage>,
}

impl ConsensusInner {
    fn new(tx: UnboundedSender<ConsensusHandleMessage>) -> Self {
        Self {
            to_manager_tx: tx,
        }
    }
}