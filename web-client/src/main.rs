mod media_selector;

use yew::prelude::*;

use media_selector::MediaSelector;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Robotic Avatar"}</h1>
            <MediaSelector />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}