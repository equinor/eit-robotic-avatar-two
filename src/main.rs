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
    utils::{
        ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType,
        Resolution,
    },
    Buffer, Camera,
};
use tokio::sync::watch;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let backends = nokhwa::query(ApiBackend::Auto).unwrap();
    println!("{backends:?}");
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
        let mut camera_a =
            Camera::new(
                CameraIndex::Index(0),
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(
                    CameraFormat::new_from(320, 180, FrameFormat::MJPEG, 30),
                )),
            )
            .unwrap();
        let mut camera_b =
            Camera::new(
                CameraIndex::Index(2),
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(
                    CameraFormat::new_from(320, 180, FrameFormat::MJPEG, 30),
                )),
            )
            .unwrap();
        camera_a.open_stream().unwrap();
        camera_b.open_stream().unwrap();
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

async fn websocket(mut socket: WebSocket, mut sight: Sight) {
    while sight.changed().await.is_ok() {
        let eyes = sight.borrow_and_update().clone();
        let mut packet = Vec::with_capacity(4 + eyes.0.buffer().len() + eyes.1.buffer().len());
        packet.extend_from_slice(&u32::to_be_bytes(eyes.0.buffer().len().try_into().unwrap()));
        packet.extend_from_slice(eyes.0.buffer());
        packet.extend_from_slice(eyes.1.buffer());

        let msg = Message::Binary(packet);

        if socket.send(msg).await.is_err() {
            break;
        }
    }
}
