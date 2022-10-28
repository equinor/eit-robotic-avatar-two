use std::{cell::RefCell, rc::Rc};

use common::Interface;
use gloo_timers::future::TimeoutFuture;
use time::OffsetDateTime;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use crate::services::Server;

pub struct Robot {
    state: Rc<RefCell<RobotState>>,
    on_change: Callback<()>,
    server: Server,
}

impl Robot {
    pub fn new(server: Server, on_change: Callback<()>) -> Robot {
        let state: Rc<RefCell<RobotState>> = Default::default();
        pull_status(server.clone(), on_change.clone(), state.clone());
        Robot {
            state,
            on_change,
            server,
        }
    }

    pub fn state(&self) -> RobotState {
        self.state.borrow().clone()
    }

    pub fn gen_token(&self) {
        let server = self.server.clone();
        let state = self.state.clone();
        let on_change = self.on_change.clone();

        spawn_local(async move {
            let token = server.get_robot_token().await;
            {
                let mut state_ref = state.borrow_mut();
                state_ref.token = Some(token)
            }
            on_change.emit(());
        });
    }

    pub fn gen_pin(&self) {
        let server = self.server.clone();
        let state = self.state.clone();
        let on_change = self.on_change.clone();

        spawn_local(async move {
            let pin = server.get_robot_pin().await;
            {
                let mut state_ref = state.borrow_mut();
                state_ref.pin = Some(pin)
            }
            on_change.emit(());
        });
    }
}

fn pull_status(server: Server, on_change: Callback<()>, state: Rc<RefCell<RobotState>>) {
    spawn_local(async move {
        loop {
            let status = server.get_robot().await;
            {
                let mut state_ref = state.borrow_mut();
                state_ref.last_seen = status.last_seen;
                state_ref.interfaces = status.interfaces;
            }
            on_change.emit(());
            TimeoutFuture::new(5_000).await;
        }
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RobotState {
    pub last_seen: Option<OffsetDateTime>,
    pub interfaces: Vec<Interface>,
    pub token: Option<String>,
    pub pin: Option<String>,
}
