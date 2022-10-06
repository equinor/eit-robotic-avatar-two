mod robotic;
mod ui;

pub use robotic::Robotic;
use ui::Props;
pub use ui::Ui;

fn main() {
    // Loading app
    let robotic = Robotic::new();

    // Load UI
    let app_root = gloo_utils::document()
        .get_element_by_id("robotic-avatar")
        .unwrap();
    let props = Props{robotic};
    yew::start_app_with_props_in_element::<Ui>(app_root, props);
}
