use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

use web_sys::{HtmlCanvasElement, HtmlVideoElement, MediaStream};
use yew::prelude::*;

use crate::{services::tracking::Track, headset};

#[derive(PartialEq, Eq, Properties, Clone)]
pub struct ViewportProps {
    pub streams: Option<(MediaStream, MediaStream)>,
}

#[function_component(Viewport)]
pub fn viewport(props: &ViewportProps) -> Html {
    let headset = use_mut_ref(|| headset::Wrapper::new());
    headset.borrow_mut().set_streams(&props.streams);

    let track = use_memo(|_| Track::default(), ());
    let canvas_ref = use_node_ref();
    let left_ref = use_node_ref();
    let right_ref = use_node_ref();
    let first_render = use_mut_ref(|| true);

    {
        let canvas_ref = canvas_ref.clone();
        let left_ref = left_ref.clone();
        let right_ref = right_ref.clone();
        use_effect(move || {
            let left = left_ref.cast().unwrap();
            let right = right_ref.cast().unwrap();

            if *first_render.borrow() {
                *first_render.borrow_mut() = false;
                let track = track.clone();
                let closure = Closure::new(move |value| track.send(value));
                setup_3d(canvas_ref.cast().unwrap(), &left, &right, &closure);
                closure.forget();
            }

            left.set_src_object(headset.borrow().left_viewport());
            right.set_src_object(headset.borrow().right_viewport());
            || ()
        });
    }

    html! {
        <div class={"viewport"}>
            <canvas ref={canvas_ref} />
            <video autoplay={true} ref={left_ref} />
            <video autoplay={true} ref={right_ref} />
        </div>
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
