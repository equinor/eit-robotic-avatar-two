use common::{RtcMessage, RtcSession};
use futures::join;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MediaStream, RtcIceGatheringState, RtcPeerConnection, RtcRtpReceiver, RtcSdpType,
    RtcSessionDescription, RtcSessionDescriptionInit,
};
use weblog::console_log;

use super::Server;

#[derive(Clone)]
pub struct WebRtc {
    server: Server,
}

impl WebRtc {
    pub fn new(server: Server) -> WebRtc {
        WebRtc { server }
    }

    pub async fn send_video(&self, video: (MediaStream, MediaStream)) {
        let con = Connection::from_streams(&video);
        let offers = con.create_offers().await;
        console_log!(format!("{:?}", &offers));
        self.server.post_minion_post_offers(&offers).await;
        let answer = self.server.get_minion_pull_answer().await;
        console_log!(format!("{:?}", &answer));
        con.set_answers(&answer).await;
    }

    pub async fn receive(&self) -> (MediaStream, MediaStream) {
        let offers = self.server.get_minion_pull_offers().await;
        console_log!(format!("{:?}", &offers));
        let con = Connection::from_offer(&offers).await;
        let answer = con.create_answers().await;
        console_log!(format!("{:?}", &answer));
        self.server.post_minion_post_answer(&answer).await;
        con.streams()
    }
}

struct Connection {
    left: MyPeer,
    right: MyPeer,
}

impl Connection {
    pub fn from_streams(streams: &(MediaStream, MediaStream)) -> Connection {
        let left = MyPeer::from_stream(&streams.0);
        let right = MyPeer::from_stream(&streams.1);

        Connection::new(left, right)
    }

    pub async fn from_offer(offer: &RtcMessage) -> Connection {
        let (left, right) = join!(
            MyPeer::from_offer(&offer.left),
            MyPeer::from_offer(&offer.right)
        );

        Connection::new(left, right)
    }

    fn new(left: MyPeer, right: MyPeer) -> Connection {
        left.register_events("left");
        right.register_events("right");
        Connection { left, right }
    }

    pub async fn create_offers(&self) -> RtcMessage {
        let (left, right) = join!(self.left.create_offer(), self.right.create_offer());
        RtcMessage { left, right }
    }

    pub async fn create_answers(&self) -> RtcMessage {
        let (left, right) = join!(self.left.create_answer(), self.right.create_answer());
        RtcMessage { left, right }
    }

    pub async fn set_answers(&self, answer: &RtcMessage) {
        self.left.set_remote(&answer.left).await;
        self.right.set_remote(&answer.right).await;
    }

    pub fn streams(&self) -> (MediaStream, MediaStream) {
        (self.left.stream(), self.right.stream())
    }
}

pub struct MyPeer(RtcPeerConnection);

impl MyPeer {
    fn from_stream(stream: &MediaStream) -> MyPeer {
        let peer = MyPeer::new();
        for track in stream.get_tracks().iter() {
            let track = track.dyn_into().unwrap();
            peer.0.add_track_0(&track, stream);
        }
        peer
    }

    async fn from_offer(session: &RtcSession) -> MyPeer {
        let peer = MyPeer::new();
        peer.set_remote(session).await;
        peer
    }

    fn new() -> MyPeer {
        //Todo add Ice
        //iceServers: [
        //    {urls: `stun:stun.l.google.com:19302`},
        //    {urls: `stun:stun1.l.google.com:19302`},
        //    {urls: `stun:stun2.l.google.com:19302`},
        //    {urls: `stun:stun3.l.google.com:19302`},
        //    {urls: `stun:stun4.l.google.com:19302`}
        //]
        MyPeer(RtcPeerConnection::new().unwrap())
    }

    async fn create_offer(&self) -> RtcSession {
        let offer: RtcSessionDescription = JsFuture::from(self.0.create_offer())
            .await
            .unwrap()
            .dyn_into()
            .unwrap();
        let mut local = RtcSessionDescriptionInit::new(offer.type_());
        local.sdp(&offer.sdp());

        JsFuture::from(self.0.set_local_description(&local))
            .await
            .unwrap();
        while self.0.ice_gathering_state() == RtcIceGatheringState::Gathering {
            TimeoutFuture::new(100).await
        }
        self.get_local()
    }

    async fn create_answer(&self) -> RtcSession {
        let answer: RtcSessionDescription = JsFuture::from(self.0.create_answer())
            .await
            .unwrap()
            .dyn_into()
            .unwrap();
        let mut local = RtcSessionDescriptionInit::new(answer.type_());
        local.sdp(&answer.sdp());

        JsFuture::from(self.0.set_local_description(&local))
            .await
            .unwrap();
        while self.0.ice_gathering_state() != RtcIceGatheringState::Complete {
            TimeoutFuture::new(100).await
        }
        self.get_local()
    }

    fn stream(&self) -> MediaStream {
        let stream = MediaStream::new().unwrap();
        for receiver in self.0.get_receivers().iter() {
            let receiver: RtcRtpReceiver = receiver.dyn_into().unwrap();
            stream.add_track(&receiver.track());
        }
        stream
    }

    fn register_events(&self, side: &'static str) {
        let events = [
            "connectionstatechange",
            "datachannel",
            "icecandidate",
            "icecandidateerror",
            "iceconnectionstatechange",
            "icegatheringstatechange",
            "negotiationneeded",
            "signalingstatechange",
            "track",
        ];

        for event in events {
            let closure: Closure<dyn Fn(JsValue)> = Closure::new(move |e: JsValue| {
                console_log!(side, event, e);
            });
            self.0
                .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }

    fn get_local(&self) -> RtcSession {
        let session = self.0.local_description().unwrap();
        let sdp_type = match session.type_() {
            RtcSdpType::Offer => "offer",
            RtcSdpType::Pranswer => "pranswer",
            RtcSdpType::Answer => "answer",
            RtcSdpType::Rollback => "rollback",
            _ => "",
        }
        .to_string();
        RtcSession {
            sdp_type,
            sdp: session.sdp(),
        }
    }

    async fn set_remote(&self, remote: &RtcSession) {
        let type_ = RtcSdpType::from_js_value(&remote.sdp_type.as_str().into()).unwrap();
        let mut init = RtcSessionDescriptionInit::new(type_);
        init.sdp(&remote.sdp);

        JsFuture::from(self.0.set_remote_description(&init))
            .await
            .unwrap();
    }
}
