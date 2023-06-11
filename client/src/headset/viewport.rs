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
    canvas_ref: NodeRef,
    left_ref: NodeRef,
    right_ref: NodeRef,
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
        false
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast().unwrap();
            let left = self.left_ref.cast().unwrap();
            let right = self.right_ref.cast().unwrap();

            headset(&canvas, &left, &right);

            left.set_src_object(self.headset.left_viewport());
            right.set_src_object(self.headset.right_viewport());
        }
    }
}
