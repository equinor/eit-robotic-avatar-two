use futures::join;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MediaStream, RtcIceGatheringState, RtcPeerConnection, RtcRtpReceiver, RtcSessionDescription,
    RtcSessionDescriptionInit,
};
use weblog::console_log;

pub struct Connection {
    left: MyPeer,
    right: MyPeer,
}

impl Connection {
    pub fn from_streams(streams: &(MediaStream, MediaStream)) -> Connection {
        let left = MyPeer::from_stream(&streams.0);
        let right = MyPeer::from_stream(&streams.1);

        Connection::new(left, right)
    }

    pub async fn from_offer(
        offer: &(RtcSessionDescriptionInit, RtcSessionDescriptionInit),
    ) -> Connection {
        let (left, right) = join!(MyPeer::from_offer(&offer.0), MyPeer::from_offer(&offer.1));

        Connection::new(left, right)
    }

    fn new(left: MyPeer, right: MyPeer) -> Connection {
        left.register_events("left");
        right.register_events("right");
        Connection { left, right }
    }

    pub async fn create_offers(&self) -> (RtcSessionDescription, RtcSessionDescription) {
        join!(self.left.create_offer(), self.right.create_offer())
    }

    pub async fn create_answers(&self) -> (RtcSessionDescription, RtcSessionDescription) {
        join!(self.left.create_answer(), self.right.create_answer())
    }

    pub async fn set_answers(
        &self,
        answer: (RtcSessionDescriptionInit, RtcSessionDescriptionInit),
    ) {
        JsFuture::from(self.left.0.set_remote_description(&answer.0))
            .await
            .unwrap();
        JsFuture::from(self.right.0.set_remote_description(&answer.1))
            .await
            .unwrap();
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

    async fn from_offer(offer: &RtcSessionDescriptionInit) -> MyPeer {
        let peer = MyPeer::new();
        JsFuture::from(peer.0.set_remote_description(offer))
            .await
            .unwrap();
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

    async fn create_offer(&self) -> RtcSessionDescription {
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
        self.0.local_description().unwrap()
    }

    async fn create_answer(&self) -> RtcSessionDescription {
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
        self.0.local_description().unwrap()
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
}
