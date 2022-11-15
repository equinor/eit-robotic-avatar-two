use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use common::Tracking;
use parking_lot::{const_mutex, Mutex};

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

static OFFER: Mutex<String> = const_mutex(String::new());

async fn post_offer(body: String) {
    ANSWER.lock().clear();
    let mut offer = OFFER.lock();
    offer.clear();
    offer.push_str(&body);
}

async fn get_offer() -> String {
    let offer = OFFER.lock();
    if offer.is_empty() {
        "{}".to_string()
    } else {
        offer.clone()
    }
}

static ANSWER: Mutex<String> = const_mutex(String::new());

async fn post_answer(body: String) {
    OFFER.lock().clear();
    let mut answer = ANSWER.lock();
    answer.clear();
    answer.push_str(&body);
}

async fn get_answer() -> String {
    let answer = ANSWER.lock();
    if answer.is_empty() {
        "{}".to_string()
    } else {
        answer.clone()
    }
}
