mod media;
pub mod server;

pub use media::MediaService;
pub use server::send_message;

use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub struct Robotic(Rc<Inner>);

#[derive(PartialEq)]
struct Inner {
    media: MediaService,
}

impl Robotic {
    pub fn new() -> Robotic {
        let inner = Inner {
            media: MediaService::new(),
        };

        Robotic(Rc::new(inner))
    }

    pub fn media(&self) -> &MediaService {
        &self.0.media
    }
}

impl Default for Robotic {
    fn default() -> Self {
        Self::new()
    }
}
