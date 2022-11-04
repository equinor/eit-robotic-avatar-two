use std::collections::HashSet;

use common::{Interface, RobotStatus};
use gloo_timers::future::TimeoutFuture;
use time::OffsetDateTime;
use wasm_bindgen_futures::spawn_local;
use yew_agent::{Agent, Context, AgentLink, HandlerId};

use crate::services::server;

pub struct RobotAgent {
    state: RobotState,
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for RobotAgent {
    type Reach = Context<Self>;

    type Message = Msg;

    type Input = RobotAction;

    type Output = RobotState;

    fn create(link: AgentLink<Self>) -> Self {
        let on_status = link.callback(Msg::Status);
        spawn_local(async move {
            loop {
                on_status.emit(server::get_robot().await);
                TimeoutFuture::new(5_000).await;
            }
        });

        RobotAgent {
            state: RobotState::default(),
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Status(status) => {
                self.state.last_seen = status.last_seen;
                self.state.interfaces = status.interfaces;
                self.send_state();
            },
            Msg::Token(token) => {
                self.state.token = Some(token);
                self.send_state();
            },
            Msg::Pin(pin) => {
                self.state.pin = Some(pin);
                self.send_state()
            },
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            RobotAction::GenToken => self.link.send_future(async move {
                Msg::Token(server::get_robot_token().await)
            }),
            RobotAction::GenPin => self.link.send_future(async move {
                Msg::Pin(server::get_robot_pin().await)
            }),
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl RobotAgent {
    fn send_state(&self) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, self.state.clone());
        }
    }
}



pub enum Msg {
    Status(RobotStatus),
    Token(String),
    Pin(String),
}

pub enum RobotAction{
    GenToken,
    GenPin,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RobotState {
    pub last_seen: Option<OffsetDateTime>,
    pub interfaces: Vec<Interface>,
    pub token: Option<String>,
    pub pin: Option<String>,
}