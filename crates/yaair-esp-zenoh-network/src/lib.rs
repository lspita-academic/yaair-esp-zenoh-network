pub(self) mod atomic;
pub(self) mod message;

use std::{marker::PhantomData, sync::Arc};

use yaair::yaair::{
    messages::{inbound::InboundMessage, serializer::Serializer},
    network::Network,
};
use zenoh_pico::{keyexpr::KeyExpr, result::ZenohResult, session::Session, zid::ZId};

use crate::message::{AtomicMessagesStore, MessagePublisher, MessageSubscriber};

pub struct NetworkContext<S> {
    messages: AtomicMessagesStore,
    serializer: S,
}

pub struct ZenohPicoNetwork<'a, S> {
    messages_publisher: MessagePublisher,
    _messages_subscriber: MessageSubscriber, // store it to keep it alive
    context: Arc<NetworkContext<S>>,
    // the session should outlive the network to prevent being closed prematurely
    _phantom: PhantomData<&'a Session>,
}

/// NOTE: sample source info api is marked as unstable, so instead we send the
/// session zid in the payload to recognize peers.
impl<'a, S: Serializer> ZenohPicoNetwork<'a, S> {
    pub fn new(session: &'a Session, base_keyexpr: &KeyExpr, serializer: S) -> ZenohResult<Self> {
        let context = Arc::new(NetworkContext {
            messages: AtomicMessagesStore::new(),
            serializer,
        });

        let messages_publisher = MessagePublisher::new(session, &base_keyexpr)?;
        let messages_subscriber = MessageSubscriber::new(session, &base_keyexpr, context.clone())?;

        Ok(Self {
            messages_publisher,
            _messages_subscriber: messages_subscriber,
            context,
            _phantom: PhantomData,
        })
    }
}

impl<S: Serializer> Network<ZId, S> for ZenohPicoNetwork<'_, S> {
    fn prepare_outbound(&mut self, outbound_message: Vec<u8>) {
        if let Err(e) = self
            .messages_publisher
            .put(outbound_message, &self.context.serializer)
        {
            let zid = self.messages_publisher.zid();
            log::warn!("Error sending message from {zid}: {e}")
        }
    }

    fn prepare_inbound(&mut self) -> InboundMessage<ZId> {
        todo!()
    }
}
