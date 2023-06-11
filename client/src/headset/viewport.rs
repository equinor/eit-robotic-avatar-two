use web_sys::MediaStream;
use yew::prelude::*;

use crate::headset;

use super::headset;

#[derive(PartialEq, Eq, Properties, Clone)]
pub struct ViewportProps {
    pub streams: Option<(MediaStream, MediaStream)>,
}

pub struct Viewport {
    headset: headset::Wrapper,
    root_ref: NodeRef,
}

impl Component for Viewport {
    type Message = ();

    type Properties = ViewportProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut headset = headset::Wrapper::new();
        headset.set_streams(&ctx.props().streams);

        Viewport {
            headset,
            root_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.root_ref.clone()}>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.headset.set_streams(&ctx.props().streams);
        false
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let root = self.root_ref.cast().unwrap();
            headset(&root, &self.headset);
        }
    }
}
