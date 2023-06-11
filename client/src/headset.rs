mod viewport;

pub use viewport::*;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlCanvasElement, HtmlElement, HtmlVideoElement, MediaStream};

use crate::services::tracking::Track;

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

fn headset(root: &HtmlElement, wrapper: &Wrapper) {
    root.set_class_name("viewport");

    let document = window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into()
        .unwrap();
    root.append_child(&canvas).unwrap();

    let left: HtmlVideoElement = document
        .create_element("video")
        .unwrap()
        .dyn_into()
        .unwrap();
    left.set_autoplay(true);
    left.set_src_object(wrapper.left_viewport());
    root.append_child(&left).unwrap();

    let right: HtmlVideoElement = document
        .create_element("video")
        .unwrap()
        .dyn_into()
        .unwrap();
    right.set_autoplay(true);
    right.set_src_object(wrapper.right_viewport());
    root.append_child(&right).unwrap();

    let track = Track::default();
    let closure = Closure::new(move |value| track.send(value));
    setup_3d(&canvas, &left, &right, &closure);
    closure.forget();
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
