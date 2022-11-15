use common::{Drive, Head};
use parking_lot::Mutex;

pub struct Minion {
    movement: Mutex<(Head, Drive)>,
}

impl Minion {
    pub fn new() -> Self {
        Self {
            movement: Default::default(),
        }
    }

    pub fn movement(&self) -> (Head, Drive) {
        *self.movement.lock()
    }

    pub fn movement_set(&self, head: Head, drive: Drive) {
        *self.movement.lock() = (head, drive)
    }
}

impl Default for Minion {
    fn default() -> Self {
        Self::new()
    }
}
