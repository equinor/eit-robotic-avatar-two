use common::{Drive, Head};
use parking_lot::Mutex;

pub struct Minion {
    movement: Mutex<(Head, Drive)>,
    offer: Mutex<String>,
    answer: Mutex<String>,
}

impl Minion {
    pub fn new() -> Self {
        Self {
            movement: Default::default(),
            offer: Default::default(),
            answer: Default::default(),
        }
    }

    pub fn movement(&self) -> (Head, Drive) {
        *self.movement.lock()
    }

    pub fn movement_set(&self, head: Head, drive: Drive) {
        *self.movement.lock() = (head, drive)
    }

    pub fn offer(&self) -> String {
        let offer = self.offer.lock();
        if offer.is_empty() {
            "{}".to_string()
        } else {
            offer.clone()
        }
    }

    pub fn set_offer(&self, offer: String) {
        {
            self.answer.lock().clear();
        }
        *self.offer.lock() = offer;
    }

    pub fn answer(&self) -> String {
        let answer = self.answer.lock();
        if answer.is_empty() {
            "{}".to_string()
        } else {
            answer.clone()
        }
    }

    pub fn set_answer(&self, answer: String) {
        {
            self.offer.lock().clear();
        }
        *self.answer.lock() = answer;
    }
}

impl Default for Minion {
    fn default() -> Self {
        Self::new()
    }
}
