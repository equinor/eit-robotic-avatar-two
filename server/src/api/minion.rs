use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use common::Tracking;

use crate::Robotic;

pub fn routes(router: Router) -> Router {
    router
        .route("/api/minion/post_offer", post(post_offer))
        .route("/api/minion/get_offer", get(get_offer))
        .route("/api/minion/post_answer", post(post_answer))
        .route("/api/minion/get_answer", get(get_answer))
        .route(
            "/api/minion/tracking",
            get(tracking_get).post(tracking_post),
        )
}

async fn tracking_get(Extension(service): Extension<Robotic>) -> Json<Tracking> {
    let (head, drive) = service.minion().movement();
    Json(Tracking { head, drive })
}

async fn tracking_post(Extension(service): Extension<Robotic>, Json(tracking): Json<Tracking>) {
    service.minion().movement_set(tracking.head, tracking.drive)
}

async fn post_offer(Extension(service): Extension<Robotic>, body: String) {
    service.minion().set_offer(body)
}

async fn get_offer(Extension(service): Extension<Robotic>) -> String {
    service.minion().offer()
}

async fn post_answer(Extension(service): Extension<Robotic>, body: String) {
    service.minion().set_answer(body)
}

async fn get_answer(Extension(service): Extension<Robotic>) -> String {
    service.minion().answer()
}
