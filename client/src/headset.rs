use web_sys::MediaStream;

pub struct Wrapper {
    streams: Option<(MediaStream, MediaStream)>
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper {streams: None}
    }

    pub fn set_streams(&mut self, streams: &Option<(MediaStream, MediaStream)>) {
        self.streams = streams.clone();
    }

    pub fn left_viewport(&self) -> Option<&MediaStream> {
        self.streams.as_ref().map(|s| &s.0)
    }

    pub fn right_viewport(&self) -> Option<&MediaStream> {
        self.streams.as_ref().map(|s| &s.1)
    }
}