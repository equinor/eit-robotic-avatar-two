use axum::{extract::Query, routing::get, Extension, Json, Router};
use common::{Message, SendMessage};
use log::info;
use serde::Deserialize;
use uuid::Uuid;

use crate::Robotic;

pub fn routes(router: Router) -> Router {
    router.route("/api/messaging", get(query_message).post(post_message))
}

#[derive(Deserialize)]
struct MessageQuery {
    topics: Option<Vec<String>>,
    from_id: Option<Uuid>,
}

async fn query_message(
    Extension(service): Extension<Robotic>,
    Query(params): Query<MessageQuery>,
) -> Json<Vec<Message>> {
    Json(service.messaging().query(params.topics, params.from_id))
}

async fn post_message(Extension(service): Extension<Robotic>, Json(message): Json<SendMessage>) {
    info!(
        "New message {} to {} with {} ",
        message.msg_type, message.topic, message.payload
    );
    service.messaging().put(message);
}
