use std::rc::Rc;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

use web_sys::{HtmlCanvasElement, HtmlVideoElement, MediaStream};
use yew::prelude::*;

use crate::{headset, services::tracking::Track};

#[derive(PartialEq, Eq, Properties, Clone)]
pub struct ViewportProps {
    pub streams: Option<(MediaStream, MediaStream)>,
}

pub struct Viewport {
    headset: headset::Wrapper,
    canvas_ref: NodeRef,
    left_ref: NodeRef,
    right_ref: NodeRef,
    track: Rc<Track>,
}

impl Component for Viewport {
    type Message = ();

    type Properties = ViewportProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut headset = headset::Wrapper::new();
        headset.set_streams(&ctx.props().streams);

        Viewport {
            headset,
            canvas_ref: NodeRef::default(),
            left_ref: NodeRef::default(),
            right_ref: NodeRef::default(),
            track: Rc::new(Track::default()),
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

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.headset.set_streams(&ctx.props().streams);
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        let left = self.left_ref.cast().unwrap();
        let right = self.right_ref.cast().unwrap();

        if first_render {
            let track = self.track.clone();
            let closure = Closure::new(move |value| track.send(value));
            setup_3d(self.canvas_ref.cast().unwrap(), &left, &right, &closure);
            closure.forget();
        }

        left.set_src_object(self.headset.left_viewport());
        right.set_src_object(self.headset.right_viewport());
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
