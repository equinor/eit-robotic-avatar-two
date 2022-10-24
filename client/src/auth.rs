use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::Url;
use weblog::console_error;
use yew::Callback;

use crate::server::Server;

pub struct Auth {
    on_login: Callback<Server>,
}

impl Auth {
    pub fn new(on_login: Callback<Server>) -> Auth {
        let token = if let Some(token) = token_from_url() {
            LocalStorage::set("robotic_token", token.clone()).unwrap();
            Some(token)
        } else if let Ok(token) = LocalStorage::get("robotic_token") {
            Some(token)
        } else {
            None
        };

        // if token is set run the callback now.
        if let Some(token) = token {
            on_login.emit(Server::new(&token))
        }

        Auth { on_login }
    }

    pub fn nothing(&self) {
        self.on_login.emit(Server::new(""))
    }

    pub fn start_azure_ad(&self) {
        spawn_local(async move {
            let url = Server::get_auth_login().await;
            if !url.is_empty() {
                let window = web_sys::window().unwrap();
                let location = window.location();
                if let Err(err) = location.assign(&url) {
                    console_error!("Location assign error: ", err);
                }
            }
        });
    }
}

fn token_from_url() -> Option<String> {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let url = Url::new(&location.href().unwrap()).unwrap();
    url.search_params().get("token")
}
