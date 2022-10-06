mod messaging;

pub use messaging::Messaging;

use std::sync::Arc;

#[derive(Clone)]
pub struct Robotic(Arc<Inner>);

#[derive(Clone)]
struct Inner {
    messaging: Messaging,
}

impl Robotic {
    pub fn new() -> Robotic {
        let inner = Inner {
            messaging: Messaging::new(),
        };
        Robotic(Arc::new(inner))
    }

    pub fn messaging(&self) -> &Messaging {
        &self.0.messaging
    }

    pub fn messaging_owned(&self) -> Messaging {
        self.0.messaging.clone()
    }
}

impl Default for Robotic {
    fn default() -> Self {
        Self::new()
    }
}
