mod media;
mod media_selector;

use stylist::{
    css,
    yew::{styled_component, Global},
};
use yew::prelude::*;

use media_selector::MediaSelector;

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
                grid-template-columns: min-content 1fr;
                grid-template-rows: min-content 1fr min-content min-content;
                grid-template-areas: 
                    "header header"
                    "robot-menu robot"
                    "debug-menu debug-menu"
                    "debug debug"
            }
        "#
    );

    html! {
        <>
            <Global css={global_css} />
            <Header />
            <RobotMenu />
            <Robot/>
            <DebugMenu />
            <Debug />
        </>
    }
}

#[styled_component(Header)]
fn header() -> Html {
    let css = css!(
        r#"
        grid-area: header;
    "#
    );

    html!(<h1 class={css}>{"Robotic Avatar"}</h1>)
}

#[styled_component(RobotMenu)]
fn robot_menu() -> Html {
    let css = css!(
        r#"
        grid-area: robot-menu;
    "#
    );

    html!(<div class={css}></div>)
}

#[styled_component(Robot)]
fn robot() -> Html {
    let css = css!(
        r#"
        grid-area: robot;
    "#
    );

    html!(<div class={css}></div>)
}

#[styled_component(DebugMenu)]
fn debug_menu() -> Html {
    let css = css!(
        r#"
        grid-area: debug-menu;
    "#
    );

    html!(<div class={css}>{"Debug tools: "}</div>)
}

#[styled_component(Debug)]
fn debug() -> Html {
    let css = css!(
        r#"
        grid-area: debug;
    "#
    );

    html!(<div class={css}><MediaSelector /></div>)
}

fn main() {
    let app_root = gloo_utils::document()
        .get_element_by_id("robotic-avatar")
        .unwrap();
    yew::start_app_in_element::<App>(app_root);
}
