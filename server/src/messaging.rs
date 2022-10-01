use std::{collections::VecDeque, sync::Arc};

use anyhow::Result;
use axum::{routing::post, Extension, Json, Router};
use common::{Message, SendMessage};
use log::info;
use parking_lot::Mutex;
use uuid::Uuid;

#[derive(Clone)]
struct MessagingService {
    queue: Arc<Mutex<VecDeque<Message>>>,
}

impl MessagingService {
    pub fn new() -> MessagingService {
        MessagingService {
            queue: Default::default(),
        }
    }

    pub fn put(&mut self, msg: SendMessage) {
        let msg = Message {
            id: Uuid::new_v4(),
            topic: msg.topic,
            msg_type: msg.msg_type,
            payload: msg.payload,
        };

        let mut queue = self.queue.lock();
        queue.push_back(msg);
    }
}

pub async fn setup() -> Result<Router> {
    let service = MessagingService::new();
    let router = Router::new()
        .route("/", post(post_message))
        .layer(Extension(service));

    Ok(router)
}

async fn post_message(
    Extension(mut service): Extension<MessagingService>,
    Json(message): Json<SendMessage>,
) {
    info!(
        "New message {} to {} with {} ",
        message.msg_type, message.topic, message.payload
    );
    service.put(message);
}
