mod agents;
mod auth;
mod services;
mod ui;

pub use ui::Ui;

fn main() {
    let app_root = gloo_utils::document()
        .get_element_by_id("robotic-avatar")
        .unwrap();
    yew::start_app_in_element::<Ui>(app_root);
}
