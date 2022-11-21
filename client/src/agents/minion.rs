use common::Tracking;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::services::server;

pub struct MinionAgent {
    sending: bool,
    link: AgentLink<Self>,
}

impl Agent for MinionAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = MinionAction;
    type Output = ();

    fn create(link: AgentLink<Self>) -> Self {
        MinionAgent {
            sending: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::ReadyToSend => {
                self.sending = false;
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            MinionAction::Tracking(value) => {
                if !self.sending {
                    self.sending = true;

                    self.link.send_future(async move {
                        server::post_minion_tracking(&value).await;
                        Msg::ReadyToSend
                    });
                }
            }
        }
    }
}

pub enum Msg {
    ReadyToSend,
}

pub enum MinionAction {
    Tracking(Tracking),
}
