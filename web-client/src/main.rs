mod media;
mod media_selector;
mod messaging_debug;
mod server;

use yew::prelude::*;

use media_selector::MediaSelector;
use messaging_debug::MessagingDebug;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Robotic Avatar"}</h1>
            <MediaSelector />
            <MessagingDebug />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
