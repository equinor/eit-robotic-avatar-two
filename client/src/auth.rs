use gloo_console::error;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::Url;
use yew::Callback;

use crate::services::server;

pub struct Auth {
    on_login: Callback<()>,
}

impl Auth {
    pub fn new(on_login: Callback<()>) -> Auth {
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
            server::set_token(&token);
            on_login.emit(())
        }

        Auth { on_login }
    }

    pub fn pin(&self, pin: String) {
        let on_login = self.on_login.clone();
        spawn_local(async move {
            let token = server::post_auth_pin(pin).await;
            if !token.is_empty() {
                LocalStorage::set("robotic_token", token.clone()).unwrap();
                server::set_token(&token);
                on_login.emit(())
            }
        });
    }

    pub fn start_azure_ad(&self) {
        spawn_local(async move {
            let url = server::get_auth_login().await;
            if !url.is_empty() {
                let window = web_sys::window().unwrap();
                let location = window.location();
                if let Err(err) = location.assign(&url) {
                    error!("Location assign error: ", err);
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
