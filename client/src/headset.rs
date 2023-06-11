mod viewport;

pub use viewport::*;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, HtmlVideoElement, MediaStream};

pub struct Wrapper {
    left: MediaStream,
    right: MediaStream,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper {
            left: MediaStream::new().unwrap(),
            right: MediaStream::new().unwrap(),
        }
    }

    pub fn set_streams(&mut self, streams: &Option<(MediaStream, MediaStream)>) {
        for track in self.left.get_tracks().iter() {
            self.left.remove_track(track.dyn_ref().unwrap())
        }
        for track in self.right.get_tracks().iter() {
            self.right.remove_track(track.dyn_ref().unwrap())
        }
        if let Some((new_left, new_right)) = streams {
            for track in new_left.get_tracks().iter() {
                self.left.add_track(track.dyn_ref().unwrap())
            }
            for track in new_right.get_tracks().iter() {
                self.right.add_track(track.dyn_ref().unwrap())
            }
        }
    }

    pub fn left_viewport(&self) -> Option<&MediaStream> {
        Some(&self.left)
    }

    pub fn right_viewport(&self) -> Option<&MediaStream> {
        Some(&self.right)
    }
}

fn headset(
    canvas: &HtmlCanvasElement,
    left: &HtmlVideoElement,
    right: &HtmlVideoElement,
    on_track: &Closure<dyn FnMut(JsValue)>,
) {
    setup_3d(canvas, left, right, on_track)
}

#[wasm_bindgen(raw_module = "/js/viewport.mjs")]
extern "C" {
    fn setup_3d(
        canvas: &HtmlCanvasElement,
        left: &HtmlVideoElement,
        right: &HtmlVideoElement,
        onTrack: &Closure<dyn FnMut(JsValue)>,
    );
}
