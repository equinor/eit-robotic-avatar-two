use std::rc::Rc;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};
use web_sys::{HtmlCanvasElement, HtmlVideoElement, MediaStream};
use yew::prelude::*;

use crate::services::tracking::Track;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
}

pub enum Msg {}

pub struct Viewport {
    canvas_ref: NodeRef,
    left_ref: NodeRef,
    right_ref: NodeRef,
    track: Rc<Track>,
}

impl Component for Viewport {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Viewport {
            canvas_ref: NodeRef::default(),
            left_ref: NodeRef::default(),
            right_ref: NodeRef::default(),
            track: Default::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"viewport"}>
                <canvas ref={self.canvas_ref.clone()} />
                <video autoplay={true} ref={self.left_ref.clone()} />
                <video autoplay={true} ref={self.right_ref.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();

        let left = self.left_ref.cast().unwrap();
        let right = self.right_ref.cast().unwrap();

        if first_render {
            let track = self.track.clone();
            let closure = Closure::new(move |value| track.send(value));
            setup_3d(self.canvas_ref.cast().unwrap(), &left, &right, &closure);
            closure.forget();
        }

        left.set_src_object(props.left.as_ref());
        right.set_src_object(props.right.as_ref());
    }
}

#[wasm_bindgen(raw_module = "/js/viewport.mjs")]
extern "C" {
    fn setup_3d(
        canvas: HtmlCanvasElement,
        left: &HtmlVideoElement,
        right: &HtmlVideoElement,
        onTrack: &Closure<dyn FnMut(JsValue)>,
    );
}
