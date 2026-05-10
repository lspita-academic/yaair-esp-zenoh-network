pub mod pubsub;
pub mod store;

use serde::{Deserialize, Serialize};
use zenoh_pico::zid::ZId;

/// NOTE: the zenoh sample source info api is marked as unstable, so instead we
/// send the session zid in the payload to recognize peers.
#[derive(Serialize, Deserialize, Clone)]
pub struct MessagePacket {
    payload: Vec<u8>,
    sender: ZId,
}

impl MessagePacket {
    pub fn new(payload: Vec<u8>, sender: ZId) -> Self {
        Self { payload, sender }
    }
}
