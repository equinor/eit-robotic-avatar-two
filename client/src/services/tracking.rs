use std::{cell::RefCell, rc::Rc};

use common::{Drive, Head, Tracking};
use js_sys::Reflect;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use super::server;

#[derive(Default)]
pub struct Track {
    sending: Rc<RefCell<bool>>,
}

impl Track {
    pub fn send(&self, tracking: JsValue) {
        if *self.sending.borrow() {
            return;
        }

        *self.sending.borrow_mut() = true;
        let left = Reflect::get(&tracking, &"l".into()).unwrap();
        let tracking = Tracking {
            head: Head {
                rx: Reflect::get(&tracking, &"rx".into())
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                ry: Reflect::get(&tracking, &"ry".into())
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                rz: Reflect::get(&tracking, &"rz".into())
                    .unwrap()
                    .as_f64()
                    .unwrap(),
            },
            drive: Drive {
                speed: Reflect::get(&left, &"y".into()).unwrap().as_f64().unwrap(),
                turn: Reflect::get(&left, &"x".into()).unwrap().as_f64().unwrap(),
            },
        };

        let sending = self.sending.clone();
        spawn_local(async move {
            server::post_minion_tracking(&tracking).await;
            *sending.borrow_mut() = false;
        })
    }
}
