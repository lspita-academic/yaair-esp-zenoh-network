use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use yaair::yaair::messages::serializer::Serializer;
use zenoh_pico::{
    keyexpr::KeyExpr,
    result::{ZenohError, ZenohResult},
    sample::{Sample, SampleClosure},
    session::{
        Session,
        pubsub::{Publisher, Subscriber},
    },
    zbytes::TryIntoZBytes,
    zid::ZId,
    zvalue::{ZClone, ZClosure, ZValue},
};

use crate::{NetworkContext, atomic::PoisonedLockError};

#[derive(Serialize, Deserialize)]
pub struct MessagePacket {
    payload: Vec<u8>,
    sender: ZId,
}

impl MessagePacket {
    pub fn new(payload: Vec<u8>, sender: ZId) -> Self {
        Self { payload, sender }
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn sender(&self) -> ZId {
        self.sender
    }
}

pub struct Message {
    payload: Vec<u8>,
    timestamp: SystemTime,
}

impl Message {
    pub fn new(payload: Vec<u8>) -> Self {
        let timestamp = SystemTime::now();
        Self { payload, timestamp }
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

impl From<&MessagePacket> for Message {
    fn from(value: &MessagePacket) -> Self {
        Message::new(value.payload().to_owned())
    }
}

impl From<MessagePacket> for Message {
    fn from(value: MessagePacket) -> Self {
        Message::new(value.payload)
    }
}

pub struct AtomicMessagesStore {
    storage: Mutex<HashMap<ZId, Message>>,
}

impl AtomicMessagesStore {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(HashMap::new()),
        }
    }

    pub fn store(&self, zid: ZId, message: Message) -> Result<(), PoisonedLockError> {
        let mut storage = self.storage.lock().map_err(|_| PoisonedLockError)?;
        storage.insert(zid, message);
        Ok(())
    }
}

pub struct MessageSubscriber {
    _subscriber: Subscriber, // store it to keep it alive
}

impl MessageSubscriber {
    pub fn new<S: Serializer>(
        session: &Session,
        base_keyexpr: &KeyExpr,
        context: Arc<NetworkContext<S>>,
    ) -> ZenohResult<Self> {
        let subscriber = session.declare_subscriber(
            &base_keyexpr.join_autocanonize(&KeyExpr::new("*")?)?,
            SampleClosure::from_callback(Self::on_message::<S>, Some(context.clone()))?,
            None,
        )?;
        Ok(Self { _subscriber: subscriber })
    }

    unsafe extern "C" fn on_message<S: Serializer>(
        sample: *const <Sample as ZValue>::Value,
        context: *const NetworkContext<S>,
    ) {
        let sample = Sample::zclone(sample);
        let context = unsafe { &*context };

        let payload_bytes = sample.payload().owned_bytes();
        let packet: MessagePacket = match context.serializer.deserialize(&payload_bytes) {
            Ok(p) => p,
            Err(e) => {
                log::warn!("Failed to deserialize message packet: {e}");
                return;
            }
        };

        let zid = packet.sender();
        let message = Message::from(packet);
        if let Err(e) = context.messages.store(zid, message) {
            log::warn!("Failed to store message: {e}");
            return;
        }
    }
}

pub struct MessagePublisher {
    zid: ZId,
    publisher: Publisher,
}

#[derive(Debug, Error)]
pub enum PutError<SerializationError> {
    #[error("serialization error while trying to publish: {0}")]
    Serialization(SerializationError),
    #[error("zenoh error while trying to publish: {0}")]
    Zenoh(ZenohError),
}

impl MessagePublisher {
    pub fn new(session: &Session, base_keyexpr: &KeyExpr) -> ZenohResult<Self> {
        let zid = session.zid();
        let publisher = session.declare_publisher(
            &base_keyexpr.join_autocanonize(&KeyExpr::new(&zid.to_string())?)?,
            None,
        )?;
        Ok(Self { zid, publisher })
    }

    pub fn zid(&self) -> ZId {
        self.zid
    }

    pub fn put<S: Serializer>(
        &self,
        payload: Vec<u8>,
        serializer: &S,
    ) -> Result<(), PutError<S::Error>> {
        let packet = MessagePacket::new(payload, self.zid);
        let payload = serializer
            .serialize(&packet)
            .map_err(PutError::Serialization)
            .and_then(|v| v.try_into_zbytes().map_err(PutError::Zenoh))?;
        self.publisher.put(payload, None).map_err(PutError::Zenoh)
    }
}
