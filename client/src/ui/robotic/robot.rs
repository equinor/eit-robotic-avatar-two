use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct Props;

pub struct Robot {}

impl Component for Robot {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Robot {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Robot the tester."}</h1>
                <h2>{"Will help you with development."}</h2>
            </div>
        }
    }
}
