use futures::join;
use js_sys::Reflect;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaStream, RtcPeerConnection, RtcSessionDescriptionInit};

pub struct Connection {
    inner_js: JsConnection,
}

impl Connection {
    pub fn from_streams(streams: &(MediaStream, MediaStream)) -> Connection {
        let left = MyPeer::from_stream(&streams.0);
        let right = MyPeer::from_stream(&streams.1);

        let inner_js = JsConnection::new(left.0, right.0);
        Connection { inner_js }
    }

    pub async fn from_offer(
        offer: &(RtcSessionDescriptionInit, RtcSessionDescriptionInit),
    ) -> Connection {
        let (left, right) = join!(MyPeer::from_offer(&offer.0), MyPeer::from_offer(&offer.1));

        let inner_js = JsConnection::new(left.0, right.0);
        Connection { inner_js }
    }

    pub async fn create_offers(&self) -> JsValue {
        self.inner_js.createOffers().await
    }

    pub async fn create_answers(&self) -> JsValue {
        self.inner_js.createAnswers().await
    }

    pub async fn set_answers(&self, answer: JsValue) {
        self.inner_js.setAnswers(answer).await
    }

    pub fn streams(&self) -> (MediaStream, MediaStream) {
        let streams = self.inner_js.getStreams();
        let left = Reflect::get(&streams, &JsValue::from_str("left"))
            .unwrap()
            .dyn_into()
            .unwrap();
        let right = Reflect::get(&streams, &JsValue::from_str("right"))
            .unwrap()
            .dyn_into()
            .unwrap();
        (left, right)
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
}

#[wasm_bindgen(raw_module = "/js/modules/rtc.mjs")]
extern "C" {
    #[wasm_bindgen(js_name = Connection)]
    type JsConnection;

    #[wasm_bindgen(constructor, js_class = "Connection")]
    fn new(left: RtcPeerConnection, right: RtcPeerConnection) -> JsConnection;
    #[wasm_bindgen(method)]
    async fn createOffers(this: &JsConnection) -> JsValue;
    #[wasm_bindgen(method)]
    async fn createAnswers(this: &JsConnection) -> JsValue;
    #[wasm_bindgen(method)]
    async fn setAnswers(this: &JsConnection, answer: JsValue);
    #[wasm_bindgen(method)]
    fn getStreams(this: &JsConnection) -> JsValue;

    async fn fromOffers(offer: JsValue) -> JsValue;
}
