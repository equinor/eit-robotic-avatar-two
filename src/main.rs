use std::{sync::Arc, thread};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::{Html, Response},
    routing::get,
    Router,
};
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution},
    Buffer, Camera,
};
use serde::Serialize;
use tokio::sync::watch;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let eyes = eyes();

    transport(eyes).await;
}

type Sight = watch::Receiver<Arc<(Buffer, Buffer)>>;

fn eyes() -> watch::Receiver<Arc<(Buffer, Buffer)>> {
    let null_buffer = Buffer::new(
        Resolution {
            width_x: 0,
            height_y: 0,
        },
        &[],
        FrameFormat::RAWRGB,
    );

    let (sender, receiver) = watch::channel(Arc::new((null_buffer.clone(), null_buffer)));

    thread::spawn(move || {
        let mut camera_a = Camera::new(
            CameraIndex::Index(0),
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution),
        )
        .unwrap();
        let mut camera_b = Camera::new(
            CameraIndex::Index(1),
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution),
        )
        .unwrap();
        loop {
            let a = camera_a.frame().unwrap();
            let b = camera_b.frame().unwrap();
            sender.send(Arc::new((a, b))).unwrap();
        }
    });

    receiver
}

async fn transport(sight: Sight) {
    let app = Router::new().route("/", get(index)).route(
        "/ws",
        get({
            let sight_clone = sight.clone();
            move |ws| upgrade(ws, sight_clone)
        }),
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("./index.html"))
}

async fn upgrade(ws: WebSocketUpgrade, sight: Sight) -> Response {
    ws.on_upgrade(|socket| websocket(socket, sight))
}

#[derive(Serialize)]
struct SendPacket<'a> {
    pub eyes: (&'a [u8], &'a [u8]),
}

async fn websocket(mut socket: WebSocket, mut sight: Sight) {
    while sight.changed().await.is_ok() {
        let eyes = sight.borrow_and_update().clone();
        let packet = SendPacket {
            eyes: (eyes.0.buffer(), eyes.1.buffer()),
        };
        let msg = Message::Binary(rmp_serde::to_vec(&packet).unwrap());

        if socket.send(msg).await.is_err() {
            break;
        }
    }
}
