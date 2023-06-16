use std::sync::Arc;

use image::RgbImage;
use time::OffsetDateTime;
use tokio::sync::watch::{channel, Receiver};

pub type Camera = Receiver<Option<Picture>>;

#[derive(Debug)]
pub struct Picture {
    pub seq: u64,
    pub timestamp: OffsetDateTime,
    pub image: Arc<RgbImage>,
}

pub fn null_camera() -> Camera {
    let (_send, receive) = channel(None);
    receive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_camera_return_empty_camera() {
        let camera = null_camera();
        assert!(
            camera.borrow().is_none(),
            "Expected the value in null_camera to be None but was: {:?}",
            camera.borrow()
        );
    }
}
