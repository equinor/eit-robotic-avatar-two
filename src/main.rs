use std::time::Instant;

use nokhwa::{utils::{CameraIndex, RequestedFormat, RequestedFormatType}, Buffer, Camera, pixel_format::RgbFormat};

fn main() {
    let mut eyes = Eyes::new();

    loop {
        let start = Instant::now();
        let _a = eyes.a_frame();
        let a_done = Instant::now();
        let _b = eyes.b_frame();
        let b_done = Instant::now();

        println!("FameA: {:?} FrameB: {:?} Total: {:?}", a_done - start, b_done - a_done, b_done - start);
    }
}

struct Eyes {
    a: Camera,
    b: Camera,
}

impl Eyes {
    fn new() -> Eyes {
        let a = Camera::new(CameraIndex::Index(0), RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution)).unwrap();
        let b = Camera::new(CameraIndex::Index(1), RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution)).unwrap();
        Eyes { a,b }
    }

    fn a_frame(&mut self) -> Buffer {
        self.a.frame().unwrap()
    }

    fn b_frame(&mut self) -> Buffer {    
        self.b.frame().unwrap()
    }
}
