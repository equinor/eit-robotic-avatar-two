use std::{cell::RefCell, rc::Rc};

use common::Interface;
use gloo_timers::future::TimeoutFuture;
use time::OffsetDateTime;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use crate::server::Server;

pub struct Robot {
    state: Rc<RefCell<RobotState>>,
}

impl Robot {
    pub fn new(server: Server, on_change: Callback<()>) -> Robot {
        let state: Rc<RefCell<RobotState>> = Default::default();
        pull_status(server, on_change, state.clone());
        Robot { state }
    }

    pub fn state(&self) -> RobotState {
        self.state.borrow().clone()
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
}
