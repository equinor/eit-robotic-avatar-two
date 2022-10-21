use wasm_bindgen_futures::spawn_local;
use weblog::console_error;
use yew::Callback;

use crate::server::Server;

pub struct Auth {
    on_login: Callback<Server>,
}

impl Auth {
    pub fn new(on_login: Callback<Server>) -> Auth {
        Auth { on_login }
    }

    pub fn nothing(&self) {
        self.on_login.emit(Server::new())
    }

    pub fn start_azure_ad(&self) {
        let server = Server::new();
        spawn_local(async move {
            let url = server.get_auth_login().await;
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
