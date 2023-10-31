use std::{hint, thread};

use nokhwa::{
    pixel_format::RgbFormat,
    utils::{
        ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType,
    },
    Camera,
};
use parking_lot::Mutex;
use tiny_http::{Header, Method, Request, Response, StatusCode};
use tungstenite::{handshake::derive_accept_key, protocol::Role, Message, WebSocket};

static SIGHT: Mutex<Option<Vec<u8>>> = Mutex::new(None);

fn main() {
    eyes();
    transport();
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

fn transport() {
    let server = tiny_http::Server::http("0.0.0.0:3000").unwrap();

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };

        match (request.method(), request.url()) {
            (Method::Get, "/") => html(request, include_str!("./index.html")),
            (Method::Get, "/app.js") => javascript(request, include_str!("./app.js")),
            (Method::Get, "/viewport.js") => javascript(request, include_str!("./viewport.js")),
            (Method::Get, "/ws") => ws_update(request),
            _ => not_found(request),
        }
    }
}

fn html(req: Request, body: &str) {
    let mut res = Response::from_string(body);
    res.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
    req.respond(res).expect("response");
}

fn javascript(req: Request, body: &str) {
    let mut res = Response::from_string(body);
    res.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/javascript"[..]).unwrap());
    req.respond(res).expect("response");
}

fn ws_update(request: Request) {
    // checking the "Upgrade" header to check that it is a websocket
    if request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Upgrade"))
        .and_then(|hdr| {
            if hdr.value == "websocket" {
                Some(hdr)
            } else {
                None
            }
        })
        .is_none()
    {
        // sending the HTML page
        not_found(request);
        return;
    };

    // getting the value of Sec-WebSocket-Key
    let key = match request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Sec-WebSocket-Key"))
        .map(|h| h.value.clone())
    {
        None => {
            let response = tiny_http::Response::new_empty(tiny_http::StatusCode(400));
            request.respond(response).expect("Responded");
            return;
        }
        Some(k) => k,
    };

    // building the "101 Switching Protocols" response
    let response = tiny_http::Response::new_empty(tiny_http::StatusCode(101))
        .with_header("Upgrade: websocket".parse::<tiny_http::Header>().unwrap())
        .with_header("Connection: Upgrade".parse::<tiny_http::Header>().unwrap())
        .with_header(
            format!(
                "Sec-WebSocket-Accept: {}",
                derive_accept_key(key.as_bytes())
            )
            .parse::<tiny_http::Header>()
            .unwrap(),
        );

    let stream = request.upgrade("websocket", response);

    let mut socket = WebSocket::from_raw_socket(stream, Role::Server, Default::default());

    thread::spawn(move || loop {
        let msg = socket.read().unwrap();
        match msg {
            Message::Text(s) if s == "f" => {}
            _ => continue,
        }

        loop {
            let maybe_buffer = SIGHT.lock().take();
            if let Some(buffer) = maybe_buffer {
                let msg = Message::Binary(buffer);
                socket.send(msg).unwrap();
                break;
            }
            hint::spin_loop();
        }
    });
}

fn not_found(req: Request) {
    req.respond(Response::empty(StatusCode(404)))
        .expect("response")
}
