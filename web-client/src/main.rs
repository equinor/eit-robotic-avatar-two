use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! { "Hello world" }
}

fn main() {
    yew::start_app::<App>();
}