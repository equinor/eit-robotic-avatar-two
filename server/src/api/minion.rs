use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use common::{Drive, Head, Tracking};
use parking_lot::{const_mutex, Mutex};

pub fn routes(router: Router) -> Router {
    let minion = Minion::default();
    router
        .route("/api/minion/post_offer", post(post_offer))
        .route("/api/minion/get_offer", get(get_offer))
        .route("/api/minion/post_answer", post(post_answer))
        .route("/api/minion/get_answer", get(get_answer))
        .route(
            "/api/minion/tracking",
            get(tracking_get).post(tracking_post),
        )
        .layer(Extension(minion))
}

#[derive(Clone, Default)]
struct Minion {
    movement: Arc<Mutex<(Head, Drive)>>,
}

impl Minion {
    pub fn movement(&self) -> (Head, Drive) {
        *self.movement.lock()
    }

    pub fn movement_set(&mut self, head: Head, drive: Drive) {
        *self.movement.lock() = (head, drive)
    }
}

async fn tracking_get(Extension(minion): Extension<Minion>) -> Json<Tracking> {
    let (head, drive) = minion.movement();
    Json(Tracking { head, drive })
}

async fn tracking_post(Extension(mut minion): Extension<Minion>, Json(tracking): Json<Tracking>) {
    minion.movement_set(tracking.head, tracking.drive)
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
