use common::{RtcIce, RtcMessage, RtcSession};
use futures::join;
use gloo_console::log;
use gloo_timers::future::TimeoutFuture;
use js_sys::Array;
use url::Url;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MediaStream, RtcConfiguration, RtcIceGatheringState, RtcIceServer, RtcPeerConnection,
    RtcRtpReceiver, RtcSdpType, RtcSessionDescription, RtcSessionDescriptionInit,
};

use super::server::{self, get_minion_ice};

pub async fn send_video(video: (MediaStream, MediaStream)) {
    let con = Connection::from_streams(&video).await;
    let offers = con.create_offers().await;
    log!(format!("{:?}", &offers));
    server::post_minion_post_offers(&offers).await;
    let answer = server::get_minion_pull_answer().await;
    log!(format!("{:?}", &answer));
    con.set_answers(&answer).await;
}

pub async fn receive() -> (MediaStream, MediaStream) {
    let offers = server::get_minion_pull_offers().await;
    log!(format!("{:?}", &offers));
    let con = Connection::from_offer(&offers).await;
    let answer = con.create_answers().await;
    log!(format!("{:?}", &answer));
    server::post_minion_post_answer(&answer).await;
    con.streams()
}

struct Connection {
    left: MyPeer,
    right: MyPeer,
}

impl Connection {
    pub async fn from_streams(streams: &(MediaStream, MediaStream)) -> Connection {
        let config = config_from_ice(get_minion_ice().await);
        let left = MyPeer::from_stream(&streams.0, &config);
        let right = MyPeer::from_stream(&streams.1, &config);

        Connection::new(left, right)
    }

    pub async fn from_offer(offer: &RtcMessage) -> Connection {
        let config = config_from_ice(get_minion_ice().await);
        let (left, right) = join!(
            MyPeer::from_offer(&offer.left, &config),
            MyPeer::from_offer(&offer.right, &config)
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
    fn from_stream(stream: &MediaStream, config: &RtcConfiguration) -> MyPeer {
        let peer = MyPeer::new(config);
        for track in stream.get_tracks().iter() {
            let track = track.dyn_into().unwrap();
            peer.0.add_track_0(&track, stream);
        }
        peer
    }

    async fn from_offer(session: &RtcSession, config: &RtcConfiguration) -> MyPeer {
        let peer = MyPeer::new(config);
        peer.set_remote(session).await;
        peer
    }

    fn new(config: &RtcConfiguration) -> MyPeer {
        MyPeer(RtcPeerConnection::new_with_configuration(config).unwrap())
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
        while self.0.ice_gathering_state() != RtcIceGatheringState::Complete {
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
                log!(side, event, e);
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

fn config_from_ice(ice: RtcIce) -> RtcConfiguration {
    log!(format!("{:?}", &ice));
    let servers: Array = ice.0.into_iter().map(ice_from_url).collect();
    let mut config = RtcConfiguration::new();
    config.ice_servers(&servers);
    config
}

fn ice_from_url(mut url: Url) -> RtcIceServer {
    let mut ice = RtcIceServer::new();
    if let Some((credentials, path)) = url.path().split_once('@') {
        let path = path.to_string();
        let (username, password) = credentials.split_once(':').unwrap();
        ice.username(username);
        ice.credential(password);
        url.set_path(&path);
    }
    ice.urls(&url.to_string().into());
    ice
}
