use stylist::css;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use web_sys::{HtmlCanvasElement, HtmlVideoElement, MediaStream};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
    pub on_track: Callback<ViewportTracking>,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let css = css!(
            r#"
            & > canvas {
                background-color: #000;
                height: 100%;
                width: 100%;
            }
            
            & > video {
                display: none;
            }
        "#
        );

        let class = classes!(props.class.clone(), css);

        html! {
            <div class={class}>
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
            let callback = props.on_track.clone();
            setup_3d(
                self.canvas_ref.cast().unwrap(),
                &left,
                &right,
                &Closure::new(move |value| callback.emit(value)),
            )
        }

        left.set_src_object(props.left.as_ref());
        right.set_src_object(props.right.as_ref());
    }
}

#[wasm_bindgen]
pub struct ViewportTracking {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
    pub l: Controller,
    pub r: Controller,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Controller {
    pub x: f64,  // Thumb Sticks X
    pub y: f64,  // Thumb Sticks X
    pub a: bool, // A or X button
    pub b: bool, // B or Y button
    pub c: f64,  // Trigger
    pub d: f64,  // Grip
}

#[wasm_bindgen(raw_module = "/js/view/Viewport.mjs")]
extern "C" {
    fn setup_3d(
        canvas: HtmlCanvasElement,
        left: &HtmlVideoElement,
        right: &HtmlVideoElement,
        onTrack: &Closure<dyn FnMut(ViewportTracking)>,
    );
}
