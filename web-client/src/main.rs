mod media;
mod messaging_debug;
mod server;
mod views;

use stylist::{css, yew::Global};
use yew::prelude::*;

use crate::views::{DebugTools, Robotic};
use messaging_debug::MessagingDebug;

#[function_component(App)]
fn app() -> Html {
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
            <Robotic class={css!("grid-area: robotic;")}/>
            <DebugTools class={css!("grid-area: debug;")}/>
            <MessagingDebug />
        </>
    }
}

fn main() {
    let app_root = gloo_utils::document()
        .get_element_by_id("robotic-avatar")
        .unwrap();
    yew::start_app_in_element::<App>(app_root);
}
