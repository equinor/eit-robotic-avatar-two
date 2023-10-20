use std::{sync::Arc, thread, time::Instant};

use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution},
    Buffer, Camera,
};
use tokio::sync::watch;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut eyes = eyes();

    loop {
        let start = Instant::now();
        eyes.changed().await.unwrap();
        let _frames = eyes.borrow_and_update().clone();
        println!("Frame time: {:?}", start.elapsed());
    }
}

fn eyes() -> watch::Receiver<Arc<(Buffer, Buffer)>> {
    let null_buffer = Buffer::new(
        Resolution {
            width_x: 0,
            height_y: 0,
        },
        &[],
        FrameFormat::RAWRGB,
    );

    let (sender, receiver) = watch::channel(Arc::new((null_buffer.clone(), null_buffer)));

    thread::spawn(move || {
        let mut camera_a = Camera::new(
            CameraIndex::Index(0),
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution),
        )
        .unwrap();
        let mut camera_b = Camera::new(
            CameraIndex::Index(1),
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution),
        )
        .unwrap();
        loop {
            let a = camera_a.frame().unwrap();
            let b = camera_b.frame().unwrap();
            sender.send(Arc::new((a, b))).unwrap();
        }
    });

    receiver
}
