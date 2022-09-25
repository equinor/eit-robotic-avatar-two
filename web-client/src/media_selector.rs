use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::{Properties, Component, Context, Html, html};
use weblog::console_log;

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub struct MediaSelector{
}



impl Component for MediaSelector {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        spawn_local(get_user_video());
        MediaSelector{}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            "Media Selector"
        }
    }
}

async fn get_user_video() {
    let value = user_video_id("test id").await;
    console_log!(value);
}

#[wasm_bindgen(module = "/js/media_selector.js")]
extern "C" {
    async fn user_video_id(id: &str) -> JsValue;
}