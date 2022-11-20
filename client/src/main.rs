mod agents;
mod auth;
mod services;
mod ui;

pub use ui::Ui;

fn main() {
    yew::start_app::<Ui>();
}
