mod messaging;

pub use messaging::Messaging;

use std::sync::Arc;

#[derive(Clone)]
pub struct Robotic(Arc<Inner>);

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
}

impl Default for Robotic {
    fn default() -> Self {
        Self::new()
    }
}
