use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlCanvasElement, HtmlElement, HtmlVideoElement, MediaStream};

use crate::services::tracking::Track;

pub struct Headset {
    left: MediaStream,
    right: MediaStream,
}

impl Headset {
    pub fn new() -> Headset {
        Headset {
            left: MediaStream::new().unwrap(),
            right: MediaStream::new().unwrap(),
        }
    }

    pub fn set_streams(&mut self, (new_left, new_right): (MediaStream, MediaStream)) {
        for track in self.left.get_tracks().iter() {
            self.left.remove_track(track.dyn_ref().unwrap())
        }
        for track in self.right.get_tracks().iter() {
            self.right.remove_track(track.dyn_ref().unwrap())
        }
        for track in new_left.get_tracks().iter() {
            self.left.add_track(track.dyn_ref().unwrap())
        }
        for track in new_right.get_tracks().iter() {
            self.right.add_track(track.dyn_ref().unwrap())
        }
    }

    pub fn render(&self, root: &HtmlElement) {
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
        left.set_src_object(Some(&self.left));
        root.append_child(&left).unwrap();

        let right: HtmlVideoElement = document
            .create_element("video")
            .unwrap()
            .dyn_into()
            .unwrap();
        right.set_autoplay(true);
        right.set_src_object(Some(&self.right));
        root.append_child(&right).unwrap();

        let track = Track::default();
        let closure = Closure::new(move |value| track.send(value));
        setup_3d(&canvas, &left, &right, &closure);
        closure.forget();
    }
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
