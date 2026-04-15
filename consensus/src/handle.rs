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
            inner: Arc::new(ConsensusInner { to_manager_tx: tx }),
        }
    }
}

impl Handle for ConsensusHandle {
    type Msg = ConsensusHandleMessage;

    fn send(&self, msg: Self::Msg) {
        // directly unwrap (unsafe)
        self.inner.to_manager_tx.send(msg).unwrap();

        println!("Message sent!"); // unnecessary log
    }
}

#[derive(Debug)]
pub struct ConsensusInner {
    pub to_manager_tx: UnboundedSender<ConsensusHandleMessage>, // unnecessarily public
}