use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;
use yew::prelude::*;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: Classes,
}

pub struct Minion{
    node_ref: NodeRef,
}

impl Component for Minion {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Minion {
            node_ref: NodeRef::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={ctx.props().class.clone()} ref={self.node_ref.clone()}></div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            robotic_main(self.node_ref.cast().unwrap())
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn robotic_main(root_elem: HtmlElement);
}