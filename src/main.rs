use std::thread;

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
    utils::{
        ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType,
    },
    Camera,
};
use parking_lot::Mutex;
use tokio::task;

static SIGHT: Mutex<Option<Vec<u8>>> = Mutex::new(None);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    eyes();
    transport().await;
}

fn eyes() {
    let backends = nokhwa::query(ApiBackend::Auto).unwrap();
    println!("{backends:?}");

    thread::spawn(move || {
        let mut camera_a =
            Camera::new(
                CameraIndex::Index(0),
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(
                    CameraFormat::new_from(320, 240, FrameFormat::MJPEG, 30),
                )),
            )
            .unwrap();
        let mut camera_b =
            Camera::new(
                CameraIndex::Index(2),
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(
                    CameraFormat::new_from(320, 240, FrameFormat::MJPEG, 30),
                )),
            )
            .unwrap();
        camera_a.open_stream().unwrap();
        camera_b.open_stream().unwrap();
        loop {
            let a = camera_a.frame_raw().unwrap();
            let b = camera_b.frame_raw().unwrap();

            let mut packet = Vec::with_capacity(4 + a.len() + b.len());
            packet.extend_from_slice(&u32::to_be_bytes(a.len().try_into().unwrap()));
            packet.extend_from_slice(&a);
            packet.extend_from_slice(&b);

            *SIGHT.lock() = Some(packet);
        }
    });
}

async fn transport() {
    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(upgrade));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("./index.html"))
}

async fn upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(websocket)
}

async fn websocket(mut socket: WebSocket) {
    loop {
        let msg = socket.recv().await.unwrap().unwrap();
        match msg {
            Message::Text(s) if s == "f" => {}
            _ => continue,
        }

        loop {
            let maybe_buffer = SIGHT.lock().take();
            if let Some(buffer) = maybe_buffer {
                let msg = Message::Binary(buffer);
                socket.send(msg).await.unwrap();
                break;
            }
            task::yield_now().await;
        }
    }
}
