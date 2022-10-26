use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};
use web_sys::{HtmlElement, MediaStream};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
    pub left: Option<MediaStream>,
    pub right: Option<MediaStream>,
    pub on_track: Callback<JsValue>,
}

pub enum Msg {}

pub struct Viewport {
    node_ref: NodeRef,
    root: JsValue,
}

impl Component for Viewport {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Viewport {
            node_ref: NodeRef::default(),
            root: JsValue::null(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class={props.class.clone()} ref={self.node_ref.clone()}>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();

        if first_render {
            self.root = minion_root(self.node_ref.cast().unwrap());
        }

        let callback = props.on_track.clone();
        render(
            &self.root,
            props.left.clone(),
            props.right.clone(),
            &Closure::new(move |t| {
                callback.emit(t);
            }),
        )
    }
}

#[wasm_bindgen(raw_module = "/js/index.mjs")]
extern "C" {
    fn minion_root(root_elem: HtmlElement) -> JsValue;
    fn render(
        root: &JsValue,
        left: Option<MediaStream>,
        right: Option<MediaStream>,
        on_track: &Closure<dyn FnMut(JsValue)>,
    );
}
