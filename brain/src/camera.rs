use std::sync::Arc;

use image::RgbImage;
use time::OffsetDateTime;
use tokio::sync::watch::Receiver;

pub type Camera = Receiver<Picture>;

pub struct Picture {
    pub seq: u64,
    pub timestamp: OffsetDateTime,
    pub image: Arc<RgbImage>,
}

pub fn null_camera() -> Camera {
    todo!()
}
