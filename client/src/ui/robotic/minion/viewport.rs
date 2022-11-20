use common::{Drive, Head, Tracking};
use js_sys::Reflect;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};
use web_sys::{HtmlCanvasElement, HtmlVideoElement, MediaStream};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
    pub on_track: Callback<Tracking>,
}

pub enum Msg {}

pub struct Viewport {
    canvas_ref: NodeRef,
    left_ref: NodeRef,
    right_ref: NodeRef,
}

impl Component for Viewport {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Viewport {
            canvas_ref: NodeRef::default(),
            left_ref: NodeRef::default(),
            right_ref: NodeRef::default(),
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
            let callback = props.on_track.reform(|value: JsValue| {
                let left = Reflect::get(&value, &"l".into()).unwrap();

                Tracking {
                    head: Head {
                        rx: Reflect::get(&value, &"rx".into())
                            .unwrap()
                            .as_f64()
                            .unwrap(),
                        ry: Reflect::get(&value, &"ry".into())
                            .unwrap()
                            .as_f64()
                            .unwrap(),
                        rz: Reflect::get(&value, &"rz".into())
                            .unwrap()
                            .as_f64()
                            .unwrap(),
                    },
                    drive: Drive {
                        speed: Reflect::get(&left, &"y".into()).unwrap().as_f64().unwrap(),
                        turn: Reflect::get(&left, &"x".into()).unwrap().as_f64().unwrap(),
                    },
                }
            });
            let closure = Closure::new(move |value| callback.emit(value));
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
