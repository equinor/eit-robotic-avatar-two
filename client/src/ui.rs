mod debug;
mod robotic;

pub use debug::DebugTools;
pub use robotic::Robotic;

use crate::Robotic as Model;

use stylist::{css, yew::Global};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub robotic: Model
}

#[function_component(Ui)]
pub fn ui(props: &Props) -> Html {
    let global_css = css!(
        r#"
            html, body, #robotic-avatar{
                height: 100%;
                width: 100%;
                margin: 0;
            }

            #robotic-avatar {
                display: grid;
                grid-template-rows: 1fr min-content;
                grid-template-areas: 
                    "robotic"
                    "debug"
            }
        "#
    );

    html! {
        <>
            <Global css={global_css} />
            <Robotic class={css!("grid-area: robotic;")} model={props.robotic.clone()}/>
            <DebugTools class={css!("grid-area: debug;")} model={props.robotic.clone()}/>
        </>
    }
}
